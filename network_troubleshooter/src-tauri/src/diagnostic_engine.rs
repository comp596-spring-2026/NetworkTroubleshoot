use serde::{Deserialize, Serialize};

use crate::models::{
    DnsStatus, HttpStatus, InterfaceAddress, InterfaceStatus, NeighborState, ReachabilityStatus,
    RouteInfo, TcpStatus, TraceStatus,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum _ScanType {
    FullScan,
    QuickScan,
    ManualScan,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorSeverity {
    None,
    Low,
    Mid,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Layer {
    LayerOne,
    LayerTwo,
    LayerThree,
    LayerFour,
    LayerSeven,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CheckStatus {
    Pass,
    Warning,
    Fail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticMessage {
    pub layer: Layer,
    pub status: CheckStatus,
    pub error_level: ErrorSeverity,
    pub title: String,
    pub message: String,
}

pub fn scan_layer_one(outputs: &[InterfaceStatus]) -> Vec<DiagnosticMessage> {
    let mut messages = Vec::new();

    let usable: Vec<&InterfaceStatus> = outputs
        .iter()
        .filter(|iface| !iface.name.starts_with("lo") && !iface.name.starts_with("docker"))
        .collect();

    let any_up = usable.iter().any(|iface| iface.is_up);

    if any_up {
        messages.push(DiagnosticMessage {
            layer: Layer::LayerOne,
            status: CheckStatus::Pass,
            error_level: ErrorSeverity::None,
            title: "Physical Connection".to_string(),
            message: "A usable network interface is active.".to_string(),
        });
    } else {
        messages.push(DiagnosticMessage {
            layer: Layer::LayerOne,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "Physical Connection".to_string(),
            message: "No Ethernet or Wi-Fi interface appears to be active.".to_string(),
        });
    }

    messages
}

pub fn scan_layer_two(neighbors: &[NeighborState]) -> Vec<DiagnosticMessage> {
    let mut messages = Vec::new();

    if neighbors.is_empty() {
        messages.push(DiagnosticMessage {
            layer: Layer::LayerTwo,
            status: CheckStatus::Warning,
            error_level: ErrorSeverity::Mid,
            title: "Local Network".to_string(),
            message: "No neighboring devices detected on the network.".to_string(),
        });
        return messages;
    }

    let any_reachable = neighbors.iter().any(|n| n.is_reachable);

    if any_reachable {
        messages.push(DiagnosticMessage {
            layer: Layer::LayerTwo,
            status: CheckStatus::Pass,
            error_level: ErrorSeverity::None,
            title: "Local Network".to_string(),
            message: "Neighboring devices are reachable.".to_string(),
        });
    } else {
        messages.push(DiagnosticMessage {
            layer: Layer::LayerTwo,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "Local Network".to_string(),
            message: "No reachable devices found on the local network.".to_string(),
        });
    }

    messages
}

// having each commands for layer 3 checks (ip ping, ip route, ip addr)
// have their own functions for diagnostics
// and having a seperate command scan_layer_three()
// that integrates all these commands

pub fn diagnose_reachability_status(output: &ReachabilityStatus) -> DiagnosticMessage {
    if output.has_loss {
        return DiagnosticMessage {
            layer: Layer::LayerThree,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "Internet Reachability".to_string(),
            message: "Packet loss detected. The network may be unreachable or unstable."
                .to_string(),
        };
    }

    if !output.is_reasonable {
        return DiagnosticMessage {
            layer: Layer::LayerThree,
            status: CheckStatus::Warning,
            error_level: ErrorSeverity::Mid,
            title: "Internet Reachability".to_string(),
            message: "High latency detected. Network performance may be degraded.".to_string(),
        };
    }

    DiagnosticMessage {
        layer: Layer::LayerThree,
        status: CheckStatus::Pass,
        error_level: ErrorSeverity::None,
        title: "Internet Reachability".to_string(),
        message: "Network reachability and latency look normal.".to_string(),
    }
}

pub fn diagnose_ip_addr(output: &[InterfaceAddress]) -> DiagnosticMessage {
    let usable: Vec<&InterfaceAddress> = output
        .iter()
        .filter(|iface| !iface.name.starts_with("lo") && !iface.name.starts_with("docker"))
        .collect();

    if usable.is_empty() {
        return DiagnosticMessage {
            layer: Layer::LayerThree,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "IP Address".to_string(),
            message: "No usable network interface found.".to_string(),
        };
    }

    let mut has_valid_ip = false;
    let mut has_apipa = false;

    for iface in usable {
        if let Some(ipv4) = &iface.ipv4 {
            if ipv4.starts_with("169.254.") {
                has_apipa = true;
            } else {
                has_valid_ip = true;
            }
        }

        if iface.ipv6.is_some() {
            has_valid_ip = true;
        }
    }

    if has_valid_ip {
        return DiagnosticMessage {
            layer: Layer::LayerThree,
            status: CheckStatus::Pass,
            error_level: ErrorSeverity::None,
            title: "IP Address".to_string(),
            message: "A valid IP address is assigned.".to_string(),
        };
    }

    if has_apipa {
        return DiagnosticMessage {
            layer: Layer::LayerThree,
            status: CheckStatus::Warning,
            error_level: ErrorSeverity::Mid,
            title: "IP Address".to_string(),
            message: "APIPA address detected (169.254.x.x). DHCP may have failed.".to_string(),
        };
    }

    DiagnosticMessage {
        layer: Layer::LayerThree,
        status: CheckStatus::Fail,
        error_level: ErrorSeverity::High,
        title: "IP Address".to_string(),
        message: "No IP address assigned to any interface.".to_string(),
    }
}

pub fn diagnose_ip_route(output: &[RouteInfo]) -> DiagnosticMessage {
    let has_default_gateway = output
        .iter()
        .any(|route| route.is_default && route.gateway.is_some());

    if has_default_gateway {
        return DiagnosticMessage {
            layer: Layer::LayerThree,
            status: CheckStatus::Pass,
            error_level: ErrorSeverity::None,
            title: "Default Gateway".to_string(),
            message: "A default gateway is available.".to_string(),
        };
    }

    DiagnosticMessage {
        layer: Layer::LayerThree,
        status: CheckStatus::Fail,
        error_level: ErrorSeverity::High,
        title: "Default Gateway".to_string(),
        message: "No default gateway was found.".to_string(),
    }
}

// combined function
pub fn scan_layer_three(
    addr_output: &[InterfaceAddress],
    route_output: &[RouteInfo],
    reachability_output: Option<&ReachabilityStatus>,
) -> Vec<DiagnosticMessage> {
    let mut messages = Vec::new();

    messages.push(diagnose_ip_addr(addr_output));
    messages.push(diagnose_ip_route(route_output));

    if let Some(reachability) = reachability_output {
        messages.push(diagnose_reachability_status(reachability));
    }

    messages
}

// layer four - netcat
pub fn scan_layer_four(output: &TcpStatus) -> DiagnosticMessage {
    if !output.is_successful {
        return DiagnosticMessage {
            layer: Layer::LayerFour,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::Mid,
            title: "TCP Reachability".to_string(),
            message: format!("{} connection to the destination failed.", output.protocol),
        };
    }

    DiagnosticMessage {
        layer: Layer::LayerFour,
        status: CheckStatus::Pass,
        error_level: ErrorSeverity::None,
        title: "TCP Reachability".to_string(),
        message: format!(
            "{} connection to the destination was successful.",
            output.protocol
        ),
    }
}

// layer seven - application layer
pub fn diagnose_http(output: &HttpStatus) -> DiagnosticMessage {
    match output.status_code {
        None => DiagnosticMessage {
            layer: Layer::LayerSeven,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "HTTP Response".to_string(),
            message: "No HTTP response was received.".to_string(),
        },

        Some(code) if (200..=399).contains(&code) => DiagnosticMessage {
            layer: Layer::LayerSeven,
            status: CheckStatus::Pass,
            error_level: ErrorSeverity::None,
            title: "HTTP Response".to_string(),
            message: format!("HTTP response successful ({code})."),
        },

        Some(code) if (400..=499).contains(&code) => DiagnosticMessage {
            layer: Layer::LayerSeven,
            status: CheckStatus::Warning,
            error_level: ErrorSeverity::Mid,
            title: "HTTP Response".to_string(),
            message: format!("HTTP response received, but returned a client error ({code})."),
        },

        Some(code) if (500..=599).contains(&code) => DiagnosticMessage {
            layer: Layer::LayerSeven,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "HTTP Response".to_string(),
            message: format!("HTTP response received, but the server returned an error ({code})."),
        },

        Some(code) => DiagnosticMessage {
            layer: Layer::LayerSeven,
            status: CheckStatus::Warning,
            error_level: ErrorSeverity::Low,
            title: "HTTP Response".to_string(),
            message: format!("HTTP response received with uncommon status code ({code})."),
        },
    }
}

// application layer - layer 7 - dns resolution
pub fn diagnose_dns(output: &DnsStatus) -> DiagnosticMessage {
    if !output.is_successful {
        let reason = output
            .failure_reason
            .as_deref()
            .unwrap_or("Unknown DNS error");

        return DiagnosticMessage {
            layer: Layer::LayerSeven,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "DNS Resolution".to_string(),
            message: format!("DNS resolution failed: {reason}."),
        };
    }

    let resolved = if output.resolved_values.is_empty() {
        "No addresses returned".to_string()
    } else {
        output.resolved_values.join(", ")
    };

    DiagnosticMessage {
        layer: Layer::LayerSeven,
        status: CheckStatus::Pass,
        error_level: ErrorSeverity::None,
        title: "DNS Resolution".to_string(),
        message: format!("DNS resolved successfully: {resolved}."),
    }
}

// diagnose tracepath
pub fn diagnose_path(output: &TraceStatus) -> DiagnosticMessage {
    if !output.destination_reached && !output.hops.is_empty() {
        return DiagnosticMessage {
            layer: Layer::LayerThree,
            status: CheckStatus::Warning,
            error_level: ErrorSeverity::Low,
            title: "Path Trace".to_string(),
            message: "Trace did not reach the destination, but part of the path was visible."
                .to_string(),
        };
    } else if !output.destination_reached && output.hops.is_empty() {
        return DiagnosticMessage {
            layer: Layer::LayerThree,
            status: CheckStatus::Warning,
            error_level: ErrorSeverity::Mid,
            title: "Path Trace".to_string(),
            message: "Unable to trace a route to the destination.".to_string(),
        };
    } else {
        return DiagnosticMessage {
            layer: Layer::LayerThree,
            status: CheckStatus::Pass,
            error_level: ErrorSeverity::None,
            title: "Path Trace".to_string(),
            message: "Route to destination was traced successfully.".to_string(),
        };
    };
}
