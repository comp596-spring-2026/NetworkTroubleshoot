use std::{process::Command};

pub fn run_cmd(cmd: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("failed to run {cmd}: {e}"))?;

    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();

    // If it failed, include whatever we got (stderr usually has the reason)
    if !out.status.success() {
        let msg = if !stderr.is_empty() { stderr } else { stdout };
        return Err(if msg.is_empty() {
            format!("{cmd} failed with status: {}", out.status)
        } else {
            msg
        });
    }

    if !stderr.is_empty() && !stdout.is_empty() {
        Ok(format!("{stdout}\n{stderr}"))
    } else if !stderr.is_empty() {
        Ok(stderr)
    } else if !stdout.is_empty() {
        Ok(stdout)
    } else {
        Ok("Success (no output)".into())
    }
}

// layer 1
#[tauri::command]
pub fn link_state() -> Result<String,String> {
    run_cmd(
    "powershell",
    &[
        "-NoProfile",
        "-Command",
        "Get-NetAdapter | ConvertTo-Json"
    ]
)
}

// layer 2
#[tauri::command]
pub fn get_neighbors() -> Result<String,String> {
     run_cmd(
    "powershell",
    &[
        "-NoProfile",
        "-Command",
        "Get-NetNeighbor | ConvertTo-Json"
    ]
     )
}

// layer 3
#[tauri::command]
pub fn get_ipconfig() -> Result<String,String> {
    // Get-NetIPConfiguration
     run_cmd(
    "powershell",
    &[
        "-NoProfile",
        "-Command",
        "Get-NetIPConfiguration | ConvertTo-Json"
    ]
     )
}
