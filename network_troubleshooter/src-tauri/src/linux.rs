use std::{process::Command};

use crate::diagnostic_engine::{DiagnosticMessage,ErrorSeverity,Layer,CheckStatus};
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
    match run_cmd("ip", &["-j", "link"]) {
        Ok(output) => match linux_parser::parse_ip_link(&output) {
            Ok(data) => diagnostics.extend(diagnostic_engine::scan_layer_one(&data)),
            Err(e) => diagnostics.push(DiagnosticMessage {
                layer: Layer::LayerOne,
                status: CheckStatus::Fail,
                error_level: ErrorSeverity::High,
                title: "Physical Connection".to_string(),
                message: format!("Could not interpret interface status. {e}"),
            }),
        },
        Err(e) => diagnostics.push(DiagnosticMessage {
            layer: Layer::LayerOne,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "Physical Connection".to_string(),
            message: format!("Could not retrieve interface status. {e}"),
        }),
    }

    // Layer 2
    match run_cmd("ip", &["-j", "neigh"]) {
        Ok(output) => match linux_parser::parse_ip_neigh(&output) {
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
    let ip_addr_data = match run_cmd("ip", &["-j", "addr"]) {
        Ok(output) => match linux_parser::parse_ip_addr(&output) {
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

    let ip_route_data = match run_cmd("ip", &["-j", "route"]) {
        Ok(output) => match linux_parser::parse_ip_route(&output) {
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

    let ping_data = match run_cmd("ping", &["-c", "4", &host]) {
        Ok(output) => match linux_parser::parse_ping(&output) {
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
                message: "The reachability test could not be completed. This may happen when the host cannot be resolved, the network is disconnected, or outbound connectivity is unavailable.".to_string(),
            });
            eprintln!("ping failed: {e}");
            None
        }
    };

    if let (Some(ip_addr_data), Some(ip_route_data)) = (ip_addr_data.as_ref(), ip_route_data.as_ref()) {
        diagnostics.extend(diagnostic_engine::scan_layer_three(
            ip_addr_data,
            ip_route_data,
            ping_data.as_ref(),
        ));
    }

    // Layer 4
    match run_cmd("nc", &["-zv", &host, "443"]) {
        Ok(output) => match linux_parser::parse_netcat(&output) {
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
    match run_cmd("dig", &[&host, "+yaml"]) {
        Ok(output) => match linux_parser::parse_dig(&output) {
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
    match run_cmd("curl", &["-I", &url]) {
        Ok(output) => match linux_parser::parse_curl(&output, &url) {
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

    // Optional: path analysis
    match run_cmd("traceroute", &[&host]) {
        Ok(output) => match linux_parser::parse_traceroute(&output, &host) {
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