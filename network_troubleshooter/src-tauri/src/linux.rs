use std::{process::Command};

use crate::diagnostic_engine::DiagnosticMessage;
use crate::linux_parser;
use crate::diagnostic_engine;


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

// layer 1 : Physical Configuration

// ip link
#[tauri::command]
pub async fn ip_link() -> Result<String, String> {
    let output = run_cmd("ip", &["-j", "link"])?;
    let parsed = linux_parser::parse_ip_link(&output)?;
    let diagnostics = diagnostic_engine::scan_layer_one(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}

// nmcli
#[tauri::command]
pub async fn nmcli() -> Result<String, String> {
   let output = run_cmd(
    "nmcli",
    &["-t", "-f", "DEVICE,TYPE,STATE,CONNECTION", "dev", "status"],
    )?;
    let parsed = linux_parser::parse_nmcli(&output);
    Ok(format!("{parsed:#?}"))
}

// Layer 2 : Local Network

// ip neigh
#[tauri::command]
pub async fn ip_neigh() -> Result<String, String> {
    let output = run_cmd("ip", &["-j","neigh"])?;
    let parsed = linux_parser::parse_ip_neigh(&output)?;
    let diagnostics : Vec<DiagnosticMessage> = diagnostic_engine::scan_layer_two(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}


// Layer 3 : IP Configuration

// ip addr
#[tauri::command]
pub async fn ip_addr() -> Result<String,String> {
   let output =  run_cmd("ip", &["-j","addr"])?;
   let parsed = linux_parser::parse_ip_addr(&output)?;
   let diagnostics = diagnostic_engine::diagnose_ip_addr(&parsed);
   Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}

// ip route 
#[tauri::command]
pub async fn ip_route() -> Result<String,String> {
    let output = run_cmd("ip", &["-j","route"])?;
    let parsed = linux_parser::parse_ip_route(&output)?;
    let diagnostics = diagnostic_engine::diagnose_ip_route(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}

// Layer 3 : Connectivity Test

// ping
#[tauri::command]
pub async fn ping(ip : String) -> Result<String,String> {
    let output = run_cmd("ping" , &["-c","4", &ip])?;
    let parsed = linux_parser::parse_ping(&output)?;
    let diagnostics = diagnostic_engine::diagnose_reachability_status(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}

// combined layer three scan

#[tauri::command]
pub async fn run_layer_three_scan(host: String) -> Result<String, String> {
    let addr_output = run_cmd("ip", &["-j", "addr"])?;
    let addr_data = linux_parser::parse_ip_addr(&addr_output)?;

    let route_output = run_cmd("ip", &["-j", "route"])?;
    let route_data = linux_parser::parse_ip_route(&route_output)?;

    let ping_output = run_cmd("ping", &["-c", "4", &host])?;
    let ping_data = linux_parser::parse_ping(&ping_output)?;

    let diagnostics = diagnostic_engine::scan_layer_three(
        &addr_data,
        &route_data,
        Some(&ping_data),
    );

    Ok(format!("{diagnostics:#?}"))
}

// Layer 4 : Transport / Port Reachability

// nc
#[tauri::command]
pub async fn netcat(host: String) -> Result<String,String> {
   let output =  run_cmd("nc", &["-zv", &host, "443"])?;
   let parsed = linux_parser::parse_netcat(&output)?;
   let diagnostics = diagnostic_engine::scan_layer_four(&parsed);
   Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}


// Layer 7 : Application Test 

// curl
#[tauri::command]
pub async fn curl(url : String) -> Result<String,String> {
    let output = run_cmd("curl", &["-I", &url])?;
    let parsed = linux_parser::parse_curl(&output,&url)?;
    let diagnostics = diagnostic_engine::diagnose_http(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}

// Layer 7 : DNS Resolution

// dig
#[tauri::command]
pub async fn dig(host : String) -> Result<String,String> {
    let output = run_cmd("dig", &[&host , "+yaml"])?;
    let parsed = linux_parser::parse_dig(&output)?;
    let diagnostics = diagnostic_engine::diagnose_dns(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}

// Layer 3 / 4 : Path Analysis

//traceroute
#[tauri::command]
pub async fn traceroute(host: String) -> Result<String, String> {
    let output = run_cmd("traceroute", &[&host])?;
    let parsed = linux_parser::parse_traceroute(&output, &host)?;
    let diagnostics = diagnostic_engine::diagnose_path(&parsed);
    Ok(format!("{diagnostics:#?}"))
}


