use std::process::Command;

fn binary_path() -> String {
    std::env::var("CARGO_BIN_EXE_catisen").unwrap_or_else(|_| {
        if cfg!(windows) {
            "target/debug/catisen.exe".to_string()
        } else {
            "target/debug/catisen".to_string()
        }
    })
}

#[test]
fn headless_smoke_without_network() {
    let output = Command::new(binary_path())
        .args(["--headless", "--no-network"])
        .output()
        .expect("failed to start Catisen headless smoke test");

    assert!(
        output.status.success(),
        "headless smoke exited with {:?}; stderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("HEADLESS_SMOKE_OK"),
        "headless smoke marker missing; stdout: {stdout}"
    );
}