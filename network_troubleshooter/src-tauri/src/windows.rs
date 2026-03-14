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

// layer 1 : physical connection
// Get-NetAdapter
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

// layer 2 : local network
// Get-NetNeighbors
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

// layer 3 : ip configuration
// Get-NetIPConfiguration
#[tauri::command]
pub fn get_ipconfig() -> Result<String,String> {
    run_cmd(
    "powershell",
    &[
        "-NoProfile",
        "-Command",
        "Get-NetIPConfiguration" // removed piping to ConvertTo-Json 
    ]
     )
}

// Get-NetRoute
pub fn get_route() -> Result<String,String> {
    todo!()
}

// layer 3 : connectivity test
// Test-Connection
pub fn test_connection() -> Result<String,String> {
    todo!()
}

// layer 4
// test UDP / TCP / Port Reachability
// Test_NetConnection
#[tauri::command]
pub fn test_net_connection(host : String) -> Result<String,String> {
    let cmd = format!("Test-NetConnection -ComputerName {} -Port 443", host);
    run_cmd(
        "powershell",
        &[
            "-NoProfile",
            "-Command",
            &cmd
        ]
    )
}

// layer 7 
// can DNS resolve?
// Resolve-DnsName
#[tauri::command]
pub fn resolve_dns_name(host : String) -> Result<String,String> {
    let cmd = format!("Resolve-DnsName -Name {} | ConvertTo-Json", &host);
    run_cmd("powershell",
     &[
        "-NoProfile",
        "-Command",
        &cmd
     ])
}

// can fetch HTTP resources?
//Invoke-WebRequest
#[tauri::command]
pub fn invoke_web_request(url : String) -> Result<String,String> {
    let cmd = format!("Invoke-WebRequest {} -UseBasicParsing", &url);
    run_cmd("powershell",
     &[
        "-NoProfile",
        "-Command",
        &cmd
     ])
}

//layer 3 / 4 : path analysis 
pub fn tracert() -> Result<String,String> {
    todo!()
}
