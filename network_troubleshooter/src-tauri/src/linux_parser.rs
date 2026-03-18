// parser functions for linux commands

use serde::{Deserialize, Serialize};

use crate::models::{InterfaceStatus,InterfaceAddress};

// ip link
#[derive(Debug,Serialize,Deserialize)]
pub struct IPlinkRaw {
    pub ifname : String,
    pub operstate : String,
    pub link_type : String,
    pub address : Option<String>,
    pub flags : Vec<String>,
}
pub fn parse_ip_link(output: &str) -> Result<Vec<InterfaceStatus>, String> {
    // parse Json into vector 
    let raw: Vec<IPlinkRaw> =
        serde_json::from_str(output).map_err(|e| e.to_string())?;


    // dbg!(&raw);

    // parse raw vector data into common structure
    let parsed = raw.into_iter().map(|iface| InterfaceStatus {
            name: iface.ifname,
            is_up: iface.flags.iter().any(|flag| flag == "UP"),
            mac: iface.address,
    }).collect();

    Ok(parsed)
}

// ip addr
#[derive(Debug,Serialize,Deserialize)]
pub struct AddrInfoRaw {
    pub family: String,
    pub local: String,
    pub prefixlen: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpAddrRaw {
    pub ifname: String,
    pub addr_info: Vec<AddrInfoRaw>,
}

pub fn parse_ip_addr(output: &str) -> Result<Vec<InterfaceAddress>, String> {
    let raw: Vec<IpAddrRaw> =
        serde_json::from_str(output).map_err(|e| e.to_string())?;

    let parsed = raw
        .into_iter()
        .map(|iface| {
            let mut ipv4 = None;
            let mut ipv6 = None;

            for addr in iface.addr_info {
                if addr.family == "inet" {
                    ipv4 = Some(addr.local);
                } else if addr.family == "inet6" {
                    ipv6 = Some(addr.local);
                }
            }

            InterfaceAddress {
                name: iface.ifname,
                ipv4,
                ipv6,
            }
        })
        .collect();

    Ok(parsed)
}