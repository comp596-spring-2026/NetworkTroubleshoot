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

