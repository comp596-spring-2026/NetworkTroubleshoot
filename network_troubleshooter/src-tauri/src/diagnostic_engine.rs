
use serde::{Deserialize, Serialize};

use crate::models::{
    InterfaceStatus,
    NeighborState,
    InterfaceAddress,
    RouteInfo,
    ReachabilityStatus,};

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
        .filter(|iface| iface.name != "lo")
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

pub fn diagnose_reachability_status(
    output: &ReachabilityStatus
) -> DiagnosticMessage {

    if output.has_loss {
        return DiagnosticMessage { 
            layer: Layer::LayerThree,
            status: CheckStatus::Fail,
            error_level: ErrorSeverity::High,
            title: "Internet Reachability".to_string(),
            message: "Packet loss detected. The network may be unreachable or unstable.".to_string(),
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
        .filter(|iface| iface.name != "lo")
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
    let has_default_gateway = output.iter().any(|route| {
        route.is_default && route.gateway.is_some()
    });

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