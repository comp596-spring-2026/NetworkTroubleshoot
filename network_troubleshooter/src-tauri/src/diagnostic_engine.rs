use serde::{Deserialize, Serialize};

use crate::models::InterfaceStatus;

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