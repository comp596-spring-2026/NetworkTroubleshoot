// General Structure For Holding Both Windows and Linux Data

use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct InterfaceStatus {
    pub name : String,
    pub is_up : bool,
    pub mac : Option<String>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct InterfaceAddress {
    pub name: String,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteInfo {
    pub is_default : bool,
    pub gateway : Option<String>,
    pub interface : String,
    pub metric : Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub name: String,
    pub kind: String,
    pub is_connected: bool,
    pub connection: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeighborState {
    pub ip: String,
    pub interface: String,
    pub mac: Option<String>,
    pub is_reachable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpStatus {
    pub protocol : String,
    pub is_successful : bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  ReachabilityStatus { // ping
   pub has_loss : bool,
   pub is_reasonable : bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsStatus {
    pub query: String,
    pub record_type: String,
    pub resolved_values: Vec<String>,
    pub is_successful: bool,
    pub failure_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpStatus {
    pub url: String,
    pub status_code: Option<u16>,
    pub is_successful: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceHop {
    pub hop_number: u8,
    pub host: Option<String>,
    pub ip: Option<String>,
    pub latencies_ms: Vec<f64>,
    pub timed_out: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceStatus {
    pub target: String,
    pub hops: Vec<TraceHop>,
    pub destination_reached: bool,
}