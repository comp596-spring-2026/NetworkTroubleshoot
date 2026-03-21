// parser functions for windows commands

use serde::{Deserialize, Serialize};

use crate::models::{
    InterfaceStatus,
    InterfaceAddress,
    RouteInfo,
    ConnectionStatus,
    NeighborState,
    TcpStatus,
    ReachabilityStatus,
    DnsStatus,
    HttpStatus,
    TraceHop,
    TraceStatus
    };

#[derive(Debug, Serialize, Deserialize)]
pub struct NetAdapterRaw {
    pub MacAddress: String,
    pub Status: String,
    pub ifName: String,
    pub MediaType: String,
}

pub fn parse_net_adapter(output: &str) -> Result<Vec<InterfaceStatus>, String> {
    let raw: Vec<NetAdapterRaw> = if output.trim_start().starts_with('[') {
        serde_json::from_str(output).map_err(|e| e.to_string())?
    } else {
        let single: NetAdapterRaw =
            serde_json::from_str(output).map_err(|e| e.to_string())?;
        vec![single]
    };

    let parsed = raw
        .into_iter()
        .map(|adapter| InterfaceStatus {
            name: adapter.ifName,
            is_up: adapter.Status == "Up",
            mac: Some(adapter.MacAddress),
        })
        .collect();

    Ok(parsed)
}