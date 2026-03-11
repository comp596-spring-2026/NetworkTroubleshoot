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

// ip link
#[tauri::command]
pub fn ip_link() -> Result<String, String> {
    run_cmd("ip", &["-j","link"])
}

// nmcli
#[tauri::command]
pub fn nmcli() -> Result<String, String> {
    run_cmd("nmcli", &["dev", "status"])
}

// ip neigh
#[tauri::command]
pub fn ip_neigh() -> Result<String, String> {
    run_cmd("ip", &["-j","neigh"])
}

// ping
#[tauri::command]
pub fn ping(ip : String) -> Result<String,String> {
    run_cmd("ping" , &["-c","4", &ip])
}

// nc
#[tauri::command]
pub fn netcat(host: String) -> Result<String,String> {
    run_cmd("nc", &["-zv", &host, "443"])
}

// curl
#[tauri::command]
pub fn curl(url : String) -> Result<String,String> {
    run_cmd("curl", &["-I", &url])
}

// dig
#[tauri::command]
pub fn dig(host : String) -> Result<String,String> {
    run_cmd("dig", &[&host])
}

//traceroute
#[tauri::command]
pub fn traceroute(host : String) -> Result<String,String> {
    run_cmd("traceroute", &[&host])
}
