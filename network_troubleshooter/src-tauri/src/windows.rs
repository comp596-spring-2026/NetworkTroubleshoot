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
// Get-NetIPAddress
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

#[tauri::command]
pub async fn run_full_diagnostics(
    host: String,
    url: String,
) -> Result<Vec<DiagnosticMessage>, String> {
    let mut diagnostics: Vec<DiagnosticMessage> = Vec::new();

    // Layer 1
    match run_powershell("Get-NetAdapter | ConvertTo-Json -Depth 4") {
        Ok(output) => match windows_parser::parse_net_adapter(&output) {
            Ok(data) => diagnostics.extend(diagnostic_engine::scan_layer_one(&data)),
            Err(e) => diagnostics.push(DiagnosticMessage {
                layer: Layer::LayerOne,
                status: CheckStatus::Fail,
                error_level: ErrorSeverity::High,
                title: "Physical Connection".to_string(),
                message: format!("Could not interpret adapter status. {e}"),
            }),
        },
        Err(e) => diagnostics.push(DiagnosticMessage {
            layer: Layer::LayerOne,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "Physical Connection".to_string(),
            message: format!("Could not retrieve adapter information. {e}"),
        }),
    }

    // Layer 2
    match run_powershell("Get-NetNeighbor | ConvertTo-Json -Depth 4") {
        Ok(output) => match windows_parser::parse_net_neighbor(&output) {
            Ok(data) => diagnostics.extend(diagnostic_engine::scan_layer_two(&data)),
            Err(e) => diagnostics.push(DiagnosticMessage {
                layer: Layer::LayerTwo,
                status: CheckStatus::Warning,
                error_level: ErrorSeverity::Mid,
                title: "Local Network".to_string(),
                message: format!("Could not interpret neighbor information. {e}"),
            }),
        },
        Err(e) => diagnostics.push(DiagnosticMessage {
            layer: Layer::LayerTwo,
            status: CheckStatus::Warning,
            error_level: ErrorSeverity::Mid,
            title: "Local Network".to_string(),
            message: format!("Could not retrieve neighbor information. {e}"),
        }),
    }

    // Layer 3 inputs
    let ipconfig_data = match run_powershell("Get-NetIPAddress | ConvertTo-Json -Depth 4") {
        Ok(output) => match windows_parser::parse_net_ip_address(&output) {
            Ok(data) => Some(data),
            Err(e) => {
                diagnostics.push(DiagnosticMessage {
                    layer: Layer::LayerThree,
                    status: CheckStatus::Fail,
                    error_level: ErrorSeverity::High,
                    title: "IP Address".to_string(),
                    message: format!("Could not interpret IP configuration. {e}"),
                });
                None
            }
        },
        Err(e) => {
            diagnostics.push(DiagnosticMessage {
                layer: Layer::LayerThree,
                status: CheckStatus::Fail,
                error_level: ErrorSeverity::High,
                title: "IP Address".to_string(),
                message: format!("Could not retrieve IP configuration. {e}"),
            });
            None
        }
    };

    let route_data = match run_powershell("Get-NetRoute | ConvertTo-Json -Depth 4") {
        Ok(output) => match windows_parser::parse_net_route(&output) {
            Ok(data) => Some(data),
            Err(e) => {
                diagnostics.push(DiagnosticMessage {
                    layer: Layer::LayerThree,
                    status: CheckStatus::Fail,
                    error_level: ErrorSeverity::High,
                    title: "Default Gateway".to_string(),
                    message: format!("Could not interpret routing information. {e}"),
                });
                None
            }
        },
        Err(e) => {
            diagnostics.push(DiagnosticMessage {
                layer: Layer::LayerThree,
                status: CheckStatus::Fail,
                error_level: ErrorSeverity::High,
                title: "Default Gateway".to_string(),
                message: format!("Could not retrieve routing information. {e}"),
            });
            None
        }
    };

    let ping_script = format!(
        "Test-Connection -ComputerName '{}' -Count 4 | ConvertTo-Json -Depth 4",
        host
    );

    let ping_data = match run_powershell(&ping_script) {
        Ok(output) => match windows_parser::parse_test_connection(&output) {
            Ok(data) => Some(data),
            Err(e) => {
                diagnostics.push(DiagnosticMessage {
                    layer: Layer::LayerThree,
                    status: CheckStatus::Fail,
                    error_level: ErrorSeverity::High,
                    title: "Internet Reachability".to_string(),
                    message: format!("Could not interpret reachability test results. {e}"),
                });
                None
            }
        },
        Err(e) => {
            diagnostics.push(DiagnosticMessage {
                layer: Layer::LayerThree,
                status: CheckStatus::Fail,
                error_level: ErrorSeverity::High,
                title: "Internet Reachability".to_string(),
                message: "The reachability test could not be completed. This often happens when the target host cannot be resolved, DNS is unavailable, or the network is disconnected.".to_string(),
            });
            eprintln!("Test-Connection failed: {e}");
            None
        }
    };

    if let (Some(ipconfig_data), Some(route_data)) = (ipconfig_data.as_ref(), route_data.as_ref()) {
        diagnostics.extend(diagnostic_engine::scan_layer_three(
            ipconfig_data,
            route_data,
            ping_data.as_ref(),
        ));
    }

    // Layer 4
    let net_script = format!(
        "Test-NetConnection -ComputerName '{}' -Port 443 | ConvertTo-Json -Depth 4",
        host
    );

    match run_powershell(&net_script) {
        Ok(output) => match windows_parser::parse_test_net_connection(&output) {
            Ok(data) => diagnostics.push(diagnostic_engine::scan_layer_four(&data)),
            Err(e) => diagnostics.push(DiagnosticMessage {
                layer: Layer::LayerFour,
                status: CheckStatus::Fail,
                error_level: ErrorSeverity::Mid,
                title: "TCP Reachability".to_string(),
                message: format!("Could not interpret TCP connectivity results. {e}"),
            }),
        },
        Err(e) => diagnostics.push(DiagnosticMessage {
            layer: Layer::LayerFour,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::Mid,
            title: "TCP Reachability".to_string(),
            message: format!("Could not complete the TCP connectivity test. {e}"),
        }),
    }

    // Layer 7 - DNS
    let dns_script = format!(
        "Resolve-DnsName -Name '{}' | ConvertTo-Json -Depth 4",
        host
    );

    match run_powershell(&dns_script) {
        Ok(output) => match windows_parser::parse_resolve_dns(&output) {
            Ok(data) => diagnostics.push(diagnostic_engine::diagnose_dns(&data)),
            Err(e) => diagnostics.push(DiagnosticMessage {
                layer: Layer::LayerSeven,
                status: CheckStatus::Fail,
                error_level: ErrorSeverity::High,
                title: "DNS Resolution".to_string(),
                message: format!("Could not interpret DNS results. {e}"),
            }),
        },
        Err(e) => diagnostics.push(DiagnosticMessage {
            layer: Layer::LayerSeven,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "DNS Resolution".to_string(),
            message: format!("DNS lookup could not be completed. {e}"),
        }),
    }

    // Layer 7 - HTTP
    let web_script = format!(
        "$resp = Invoke-WebRequest -Uri '{}' -UseBasicParsing; \
         [pscustomobject]@{{ StatusCode = $resp.StatusCode }} | ConvertTo-Json",
        url
    );

    match run_powershell(&web_script) {
        Ok(output) => match windows_parser::parse_invoke_web_request(&output, &url) {
            Ok(data) => diagnostics.push(diagnostic_engine::diagnose_http(&data)),
            Err(e) => diagnostics.push(DiagnosticMessage {
                layer: Layer::LayerSeven,
                status: CheckStatus::Fail,
                error_level: ErrorSeverity::High,
                title: "HTTP Response".to_string(),
                message: format!("Could not interpret HTTP test results. {e}"),
            }),
        },
        Err(e) => diagnostics.push(DiagnosticMessage {
            layer: Layer::LayerSeven,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "HTTP Response".to_string(),
            message: format!("HTTP test could not be completed. {e}"),
        }),
    }

    // Optional path analysis
    match run_cmd("tracert", &[&host]) {
        Ok(output) => match windows_parser::parse_tracert(&output, &host) {
            Ok(data) => diagnostics.push(diagnostic_engine::diagnose_path(&data)),
            Err(e) => diagnostics.push(DiagnosticMessage {
                layer: Layer::LayerThree,
                status: CheckStatus::Warning,
                error_level: ErrorSeverity::Low,
                title: "Path Trace".to_string(),
                message: format!("Could not interpret path trace results. {e}"),
            }),
        },
        Err(e) => diagnostics.push(DiagnosticMessage {
            layer: Layer::LayerThree,
            status: CheckStatus::Warning,
            error_level: ErrorSeverity::Low,
            title: "Path Trace".to_string(),
            message: format!("Path trace could not be completed. {e}"),
        }),
    }

    Ok(diagnostics)
}