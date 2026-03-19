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