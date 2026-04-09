use std::process::Command;
use crate::windows_parser;
use crate::diagnostic_engine;
use diagnostic_engine::DiagnosticMessage;

fn run_cmd(cmd: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("failed to run {cmd}: {e}"))?;

    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();

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

fn run_powershell(script: &str) -> Result<String, String> {
    run_cmd(
        "powershell",
        &[
            "-NoProfile",
            "-Command",
            script,
        ],
    )
}

// layer 1 : physical connection
// Get-NetAdapter
#[tauri::command]
pub async fn link_state() -> Result<String, String> {
    let output = run_powershell("Get-NetAdapter | ConvertTo-Json -Depth 4")?;
    let parsed = windows_parser::parse_net_adapter(&output)?;
    let diagnostics = diagnostic_engine::scan_layer_one(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}

// layer 2 : local network
// Get-NetNeighbor
#[tauri::command]
pub async fn get_neighbors() -> Result<String, String> {
    let output = run_powershell("Get-NetNeighbor | ConvertTo-Json -Depth 4")?;
    let parsed = windows_parser::parse_net_neighbor(&output)?;
    let diagnostics : Vec<DiagnosticMessage> = diagnostic_engine::scan_layer_two(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    ))
}

// layer 3 : ip configuration
// Get-NetIPConfiguration
#[tauri::command]
pub async fn get_ipconfig() -> Result<String, String> {
    let output = run_powershell("Get-NetIPAddress | ConvertTo-Json -Depth 4")?;
    let parsed = windows_parser::parse_net_ip_address(&output)?;
    let diagnostics = diagnostic_engine::diagnose_ip_addr(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    )) 
}

// layer 3 : routing table
// Get-NetRoute
#[tauri::command]
pub async fn get_route() -> Result<String, String> {
    let output = run_powershell("Get-NetRoute | ConvertTo-Json -Depth 4")?;
    let parsed = windows_parser::parse_net_route(&output)?;
    let diagnostics = diagnostic_engine::diagnose_ip_route(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    )) 
}

// layer 3 : connectivity test
// Test-Connection
#[tauri::command]
pub async fn test_connection(host: String) -> Result<String, String> {
    let script = format!(
        "Test-Connection -ComputerName '{}' -Count 4 | ConvertTo-Json -Depth 4",
        host
    );
    let output = run_powershell(&script)?;
    let parsed = windows_parser::parse_test_connection(&output)?;
    let diagnostics = diagnostic_engine::diagnose_reachability_status(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    )) 
}

// layer 4
// test UDP / TCP / Port Reachability
// Test-NetConnection
#[tauri::command]
pub async fn test_net_connection(host: String) -> Result<String, String> {
    let script = format!(
        "Test-NetConnection -ComputerName '{}' -Port 443 | ConvertTo-Json -Depth 4",
        host
    );
    let output = run_powershell(&script)?;
    let parsed = windows_parser::parse_test_net_connection(&output)?;
    let diagnostics = diagnostic_engine::scan_layer_four(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    )) 
}

// layer 7
// can DNS resolve?
// Resolve-DnsName
#[tauri::command]
pub async fn resolve_dns_name(host: String) -> Result<String, String> {
    let script = format!(
        "Resolve-DnsName -Name '{}' | ConvertTo-Json -Depth 4",
        host
    );
    let output = run_powershell(&script)?;
    let parsed = windows_parser::parse_resolve_dns(&output)?;
    let diagnostics = diagnostic_engine::diagnose_dns(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    )) 
}

// can fetch HTTP resources?
// Invoke-WebRequest
#[tauri::command]
pub async fn invoke_web_request(url: String) -> Result<String, String> {
    let script = format!(
        "$resp = Invoke-WebRequest -Uri '{}' -UseBasicParsing; \
         [pscustomobject]@{{ StatusCode = $resp.StatusCode }} | ConvertTo-Json",
        url
    );

   let output =  run_powershell(&script)?;
   let parsed = windows_parser::parse_invoke_web_request(&output,&url)?;
   let diagnostics = diagnostic_engine::diagnose_http(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    )) 
}


// layer 3 / 4 : path analysis
// tracert
#[tauri::command]
pub async fn tracert(host: String) -> Result<String, String> {
    let output = run_cmd("tracert", &[&host])?;
    let parsed = windows_parser::parse_tracert(&output, &host)?;
    let diagnostics = diagnostic_engine::diagnose_path(&parsed);
    Ok(format!(
        "Parsed:\n{parsed:#?}\n\nDiagnostics:\n{diagnostics:#?}"
    )) 
}

// combined command
#[tauri::command]
pub async fn run_full_diagnostics(
    host: String,
    url: String,
) -> Result<Vec<DiagnosticMessage>, String> {
    let mut diagnostics: Vec<DiagnosticMessage> = Vec::new();

    // Layer 1
    let link_state_output = run_powershell("Get-NetAdapter | ConvertTo-Json -Depth 4")?;
    let link_state_data = windows_parser::parse_net_adapter(&link_state_output)?;
    diagnostics.extend(diagnostic_engine::scan_layer_one(&link_state_data));

    // Layer 2
    let neighbors_output = run_powershell("Get-NetNeighbor | ConvertTo-Json -Depth 4")?;
    let neighbors_data = windows_parser::parse_net_neighbor(&neighbors_output)?;
    diagnostics.extend(diagnostic_engine::scan_layer_two(&neighbors_data));

    // Layer 3
    let ipconfig_output = run_powershell("Get-NetIPConfiguration | ConvertTo-Json -Depth 6")?;
    let ipconfig_data = windows_parser::parse_net_ip_config(&ipconfig_output)?;

    let route_output = run_powershell("Get-NetRoute | ConvertTo-Json -Depth 4")?;
    let route_data = windows_parser::parse_net_route(&route_output)?;

    let ping_script = format!(
        "Test-Connection -ComputerName '{}' -Count 4 | ConvertTo-Json -Depth 4",
        host
    );
    let ping_output = run_powershell(&ping_script)?;
    let ping_data = windows_parser::parse_test_connection(&ping_output)?;

    diagnostics.extend(diagnostic_engine::scan_layer_three(
        &ipconfig_data,
        &route_data,
        Some(&ping_data),
    ));

    // Layer 4
    let net_script = format!(
        "Test-NetConnection -ComputerName '{}' -Port 443 | ConvertTo-Json -Depth 4",
        host
    );
    let net_output = run_powershell(&net_script)?;
    let net_data = windows_parser::parse_test_net_connection(&net_output)?;
    diagnostics.push(diagnostic_engine::scan_layer_four(&net_data));

    // Layer 7 - DNS
    let dns_script = format!(
        "Resolve-DnsName -Name '{}' | ConvertTo-Json -Depth 4",
        host
    );
    let dns_output = run_powershell(&dns_script)?;
    let dns_data = windows_parser::parse_resolve_dns(&dns_output)?;
    diagnostics.push(diagnostic_engine::diagnose_dns(&dns_data));

    // Layer 7 - HTTP
    let web_script = format!(
        "$resp = Invoke-WebRequest -Uri '{}' -UseBasicParsing; \
         [pscustomobject]@{{ StatusCode = $resp.StatusCode }} | ConvertTo-Json",
        url
    );
    let web_output = run_powershell(&web_script)?;
    let web_data = windows_parser::parse_invoke_web_request(&web_output, &url)?;
    diagnostics.push(diagnostic_engine::diagnose_http(&web_data));

    // Optional: path analysis
    let tracert_output = run_cmd("tracert", &[&host])?;
    let tracert_data = windows_parser::parse_tracert(&tracert_output, &host)?;
    diagnostics.push(diagnostic_engine::diagnose_path(&tracert_data));

    Ok(diagnostics)
}