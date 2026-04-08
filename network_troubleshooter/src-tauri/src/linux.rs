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
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}

// combined function to call for diagnostics

#[tauri::command]
pub async fn run_full_diagnostics(
    host: String,
    url: String,
) -> Result<Vec<DiagnosticMessage>, String> {
    let mut diagnostics: Vec<DiagnosticMessage> = Vec::new();

    // Layer 1
    let ip_link_output = run_cmd("ip", &["-j", "link"])?;
    let ip_link_data = linux_parser::parse_ip_link(&ip_link_output)?;
    diagnostics.extend(diagnostic_engine::scan_layer_one(&ip_link_data));

    // Layer 2
    let ip_neigh_output = run_cmd("ip", &["-j", "neigh"])?;
    let ip_neigh_data = linux_parser::parse_ip_neigh(&ip_neigh_output)?;
    diagnostics.extend(diagnostic_engine::scan_layer_two(&ip_neigh_data));

    // Layer 3
    let ip_addr_output = run_cmd("ip", &["-j", "addr"])?;
    let ip_addr_data = linux_parser::parse_ip_addr(&ip_addr_output)?;

    let ip_route_output = run_cmd("ip", &["-j", "route"])?;
    let ip_route_data = linux_parser::parse_ip_route(&ip_route_output)?;

    let ping_output = run_cmd("ping", &["-c", "4", &host])?;
    let ping_data = linux_parser::parse_ping(&ping_output)?;

    diagnostics.extend(diagnostic_engine::scan_layer_three(
        &ip_addr_data,
        &ip_route_data,
        Some(&ping_data),
    ));

    // Layer 4
    let netcat_output = run_cmd("nc", &["-zv", &host, "443"])?;
    let netcat_data = linux_parser::parse_netcat(&netcat_output)?;
    diagnostics.push(diagnostic_engine::scan_layer_four(&netcat_data));

    // Layer 7 - DNS
    let dig_output = run_cmd("dig", &[&host, "+yaml"])?;
    let dig_data = linux_parser::parse_dig(&dig_output)?;
    diagnostics.push(diagnostic_engine::diagnose_dns(&dig_data));

    // Layer 7 - HTTP
    let curl_output = run_cmd("curl", &["-I", &url])?;
    let curl_data = linux_parser::parse_curl(&curl_output, &url)?;
    diagnostics.push(diagnostic_engine::diagnose_http(&curl_data));

    // Optional: path analysis
    let traceroute_output = run_cmd("traceroute", &[&host])?;
    let traceroute_data = linux_parser::parse_traceroute(&traceroute_output, &host)?;
    diagnostics.push(diagnostic_engine::diagnose_path(&traceroute_data));

    Ok(diagnostics)
}


