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