use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use tokio::time::sleep;

#[derive(Debug, Clone)]
struct TestCase {
    url: &'static str,
    label: &'static str,
    use_tor: bool,
    timeout_secs: u64,
}

#[derive(Debug)]
enum TestOutcome {
    Success {
        status: u16,
        bytes: usize,
        full_load_ms: u128,
        line: String,
    },
    NetworkError(String),
    EarlyExit(Option<i32>),
    Timeout,
}

fn parse_bool_env(name: &str) -> bool {
    match std::env::var(name) {
        Ok(v) => matches!(v.trim().to_lowercase().as_str(), "1" | "true" | "yes" | "on"),
        Err(_) => false,
    }
}

fn tor_proxy_reachable() -> bool {
    use std::net::{TcpStream, ToSocketAddrs};

    let proxy = std::env::var("CATISEN_TOR_PROXY")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "socks5h://127.0.0.1:9150".to_string());

    let no_scheme = proxy.split("://").last().unwrap_or(proxy.as_str());
    let host_port = no_scheme.rsplit('@').next().unwrap_or(no_scheme);
    let mut split = host_port.rsplitn(2, ':');
    let port = split
        .next()
        .and_then(|p| p.trim().parse::<u16>().ok())
        .unwrap_or(0);
    let host = split.next().unwrap_or("127.0.0.1").trim();
    if port == 0 || host.is_empty() {
        return false;
    }

    let addr = format!("{}:{}", host, port);
    if let Ok(addrs) = addr.to_socket_addrs() {
        for candidate in addrs {
            if TcpStream::connect_timeout(&candidate, Duration::from_millis(700)).is_ok() {
                return true;
            }
        }
    }

    false
}

fn binary_path() -> String {
    match std::env::var("CARGO_BIN_EXE_catisen") {
        Ok(v) => v,
        Err(_) => {
            if cfg!(windows) {
                "target/debug/catisen.exe".to_string()
            } else {
                "target/debug/catisen".to_string()
            }
        }
    }
}

fn host_fragment(url: &str) -> String {
    let no_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    no_scheme
        .split('/')
        .next()
        .unwrap_or(no_scheme)
        .to_lowercase()
}

fn parse_req_number<T: std::str::FromStr>(line: &str, key: &str) -> Option<T> {
    let prefix = format!("{}=", key);
    line.split_whitespace()
        .find(|token| token.starts_with(&prefix))
        .and_then(|token| token[prefix.len()..].parse::<T>().ok())
}

fn parse_req_line(line: &str) -> Option<(u16, usize, u128)> {
    let status = parse_req_number::<u16>(line, "status")?;
    let bytes = parse_req_number::<usize>(line, "bytes")?;
    let full_load_ms = parse_req_number::<u128>(line, "full")?;
    Some((status, bytes, full_load_ms))
}

async fn run_case(case: &TestCase) -> TestOutcome {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let log_path = std::env::temp_dir().join(format!(
        "catisen-integration-{}-{}.log",
        case.label.replace(' ', "_"),
        timestamp
    ));

    let exe = binary_path();
    let mut cmd = Command::new(exe);
    cmd.arg("--url")
        .arg(case.url)
        .env("CATISEN_LOG_FILE", &log_path)
        .env("CATISEN_VIEW_MODE", "TextOnly")
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    if case.use_tor {
        cmd.arg("--tor");
    }

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) => panic!("failed to start browser process for {}: {}", case.label, err),
    };

    let deadline = Instant::now() + Duration::from_secs(case.timeout_secs);
    let host = host_fragment(case.url);
    let mut outcome = TestOutcome::Timeout;

    while Instant::now() < deadline {
        if let Ok(content) = std::fs::read_to_string(&log_path) {
            // Prefer a URL match for the intended host, but allow redirect/final URL cases
            // by falling back to the latest request telemetry line from this process run.
            let preferred_req = content
                .lines()
                .rev()
                .find(|line| line.starts_with("[REQ]") && line.to_lowercase().contains(&host));
            let fallback_req = content.lines().rev().find(|line| line.starts_with("[REQ]"));

            if let Some(req_line) = preferred_req.or(fallback_req) {
                if let Some((status, bytes, full_load_ms)) = parse_req_line(req_line) {
                    outcome = TestOutcome::Success {
                        status,
                        bytes,
                        full_load_ms,
                        line: req_line.to_string(),
                    };
                    break;
                }
            }

            if let Some(err_line) = content
                .lines()
                .rev()
                .find(|line| line.contains("❌ Network Error:"))
            {
                outcome = TestOutcome::NetworkError(err_line.to_string());
                break;
            }
        }

        match child.try_wait() {
            Ok(Some(status)) => {
                outcome = TestOutcome::EarlyExit(status.code());
                break;
            }
            Ok(None) => {}
            Err(_) => {}
        }

        sleep(Duration::from_millis(400)).await;
    }

    if let Ok(None) = child.try_wait() {
        let _ = child.kill();
        let _ = child.wait();
    }

    let _ = std::fs::remove_file(&log_path);
    outcome
}

#[tokio::test]
async fn test_clearnet_and_darknet() {
    let run_tor = parse_bool_env("CATISEN_RUN_TOR_TESTS");
    let require_tor_success = parse_bool_env("CATISEN_REQUIRE_TOR_SUCCESS");
    let tor_reachable = tor_proxy_reachable();
    let mut clearnet_successes = 0usize;
    let mut tor_successes = 0usize;
    let mut failures: Vec<String> = Vec::new();

    let test_cases = vec![
        TestCase {
            url: "https://www.mozilla.org",
            label: "clearnet_mozilla",
            use_tor: false,
            timeout_secs: 25,
        },
        TestCase {
            url: "https://www.wikipedia.org",
            label: "clearnet_wikipedia",
            use_tor: false,
            timeout_secs: 25,
        },
        TestCase {
            url: "http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion",
            label: "tor_onion_ahmia",
            use_tor: true,
            timeout_secs: 35,
        },
    ];

    for case in test_cases {
        if case.use_tor && run_tor && !tor_reachable {
            println!(
                "=== Skipping {} (Tor proxy endpoint is unreachable in this environment) ===",
                case.label
            );
            continue;
        }

        if case.use_tor && !run_tor {
            println!(
                "=== Skipping {} (set CATISEN_RUN_TOR_TESTS=1 to enable Tor onion test) ===",
                case.label
            );
            continue;
        }

        println!("=== Testing {}: {} ===", case.label, case.url);
        let result = run_case(&case).await;

        match result {
            TestOutcome::Success {
                status,
                bytes,
                full_load_ms,
                line,
            } => {
                println!(
                    "PASS {} | status={} | {}ms | {} bytes",
                    case.label, status, full_load_ms, bytes
                );
                println!("REQ line: {}", line);
                if status > 0 && bytes > 0 {
                    if case.use_tor {
                        tor_successes += 1;
                    } else {
                        clearnet_successes += 1;
                    }
                } else {
                    failures.push(format!(
                        "{} returned non-useful payload (status={}, bytes={})",
                        case.label, status, bytes
                    ));
                }
            }
            TestOutcome::NetworkError(e) => {
                println!("FAIL {} network error: {}", case.label, e);
                failures.push(format!("{} network error: {}", case.label, e));
            }
            TestOutcome::EarlyExit(code) => {
                println!("FAIL {} browser exited early with code {:?}", case.label, code);
                failures.push(format!(
                    "{} browser exited early with code {:?}",
                    case.label, code
                ));
            }
            TestOutcome::Timeout => {
                println!("FAIL {} timed out waiting for [REQ] telemetry", case.label);
                failures.push(format!(
                    "{} timed out waiting for [REQ] telemetry",
                    case.label
                ));
            }
        }

        sleep(Duration::from_secs(2)).await;
    }

    assert!(
        clearnet_successes >= 1,
        "No successful clearnet fetches. Failures: {:?}",
        failures
    );

    if run_tor && require_tor_success {
        assert!(
            tor_successes >= 1,
            "Tor strict mode enabled but no successful onion fetches. Failures: {:?}",
            failures
        );
    }

    if !failures.is_empty() {
        println!("Non-fatal failures observed: {:?}", failures);
    }
}