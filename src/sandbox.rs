// Catisen process hardening.
//
// This module is deliberately honest about its scope: the controls below are
// useful OS hardening, not a Chromium-equivalent renderer sandbox. The browser
// must not treat a successful call as proof that arbitrary native code is
// contained.

#[cfg(target_os = "windows")]
pub mod windows_sandbox {
    use std::mem::{size_of, zeroed};
    use std::ptr::null_mut;

    use winapi::um::jobapi2::{AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject};
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use winapi::um::winnt::{
        JobObjectExtendedLimitInformation, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    };

    /// Place the current process in a Job Object and apply a process-lifetime
    /// containment limit. The raw job handle is intentionally kept open for the
    /// lifetime of the process; closing it would release KILL_ON_JOB_CLOSE.
    ///
    /// This does not provide an AppContainer, restricted token, handle filter,
    /// or syscall sandbox. WebView2's own renderer isolation remains separate.
    pub fn engage_strict_sandbox() -> Result<(), String> {
        unsafe {
            let job = CreateJobObjectW(null_mut(), null_mut());
            if job.is_null() {
                return Err(format!(
                    "CreateJobObjectW failed: {}",
                    std::io::Error::last_os_error()
                ));
            }

            let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = zeroed();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            let set_ok = SetInformationJobObject(
                job,
                JobObjectExtendedLimitInformation,
                &mut info as *mut JOBOBJECT_EXTENDED_LIMIT_INFORMATION
                    as *mut winapi::ctypes::c_void,
                size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            );
            if set_ok == 0 {
                return Err(format!(
                    "SetInformationJobObject failed: {}",
                    std::io::Error::last_os_error()
                ));
            }

            if AssignProcessToJobObject(job, GetCurrentProcess()) == 0 {
                return Err(format!(
                    "AssignProcessToJobObject failed: {}",
                    std::io::Error::last_os_error()
                ));
            }

            // Do not call CloseHandle(job): the job must remain alive while the
            // process is running. The OS reclaims the handle at process exit.
        }

        eprintln!(
            "[Catisen] Windows Job Object hardening engaged (kill-on-job-close); this is not a full renderer sandbox."
        );
        Ok(())
    }
}

#[cfg(target_os = "linux")]
pub mod linux_sandbox {
    /// Set Linux no-new-privileges before any WebView or helper process starts.
    /// This blocks setuid/setgid and file-capability privilege gains inherited by
    /// future execs and is a prerequisite for installing an unprivileged seccomp
    /// filter. It is intentionally not described as a complete syscall sandbox.
    pub fn engage_strict_sandbox() -> Result<(), String> {
        // SAFETY: PR_SET_NO_NEW_PRIVS takes scalar arguments only; the remaining
        // arguments are unused and conventionally passed as zero.
        let rc = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
        if rc != 0 {
            return Err(format!(
                "prctl(PR_SET_NO_NEW_PRIVS) failed: {}",
                std::io::Error::last_os_error()
            ));
        }
        eprintln!(
            "[Catisen] Linux PR_SET_NO_NEW_PRIVS engaged; "
            "this is not a complete seccomp renderer sandbox."
        );
        Ok(())
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub mod generic_sandbox {
    pub fn engage_strict_sandbox() -> Result<(), String> {
        Err("no process hardening implementation is available for this platform".to_string())
    }
}

pub struct SandboxManager;

impl SandboxManager {
    pub fn lockdown_current_process() -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            windows_sandbox::engage_strict_sandbox()
        }
        #[cfg(target_os = "linux")]
        {
            linux_sandbox::engage_strict_sandbox()
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            generic_sandbox::engage_strict_sandbox()
        }
    }
}
