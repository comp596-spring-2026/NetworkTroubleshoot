// parser functions for linux commands

use serde::{Deserialize, Serialize};

use crate::models::{InterfaceStatus,
    InterfaceAddress,
    RouteInfo,
    ConnectionStatus,
    NeighborState};

// ======================= ip link =======================

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

// ======================= ip addr =======================

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

// ======================= ip route ======================= 

#[derive(Debug, Serialize, Deserialize)]
pub struct IpRouteRaw {
    pub dst: String,
    pub gateway: Option<String>,
    pub dev: String,
    pub protocol: Option<String>,
    pub metric: Option<u32>,
    pub prefsrc: Option<String>,
}

pub fn parse_ip_route(output: &str) -> Result<Vec<RouteInfo>,String>{
    let raw : Vec<IpRouteRaw> = serde_json::from_str(&output).map_err(|e| e.to_string())?;
    
    //dbg!(&raw);

    let parsed = raw.into_iter().map(|route | RouteInfo {
        is_default : route.dst == "default",
        gateway : route.gateway,
        interface : route.dev,
        metric : route.metric,
    }).collect();

    return Ok(parsed);
}

// ======================= nmcli ======================= 

#[derive(Debug, Serialize, Deserialize)]
pub struct NmcliRaw {
    device : String,
    dev_type : String,
    state : String,
    connection : Option<String>,
}

pub fn parse_nmcli(output: &str) -> Result<Vec<ConnectionStatus>, String> {
    let mut devices = Vec::new();

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() < 4 {
            return Err(format!("invalid nmcli line: {}", line));
        }

        let connection = if parts[3] == "--" {
            None
        } else {
            Some(parts[3].to_string())
        };

        let raw = NmcliRaw {
            device: parts[0].to_string(),
            dev_type: parts[1].to_string(),
            state: parts[2].to_string(),
            connection,
        };

        devices.push(raw);
    }

    let parsed = devices.into_iter().map(|each| ConnectionStatus {
            name : each.device,
            kind : each.dev_type,
            is_connected : each.state.starts_with("connected"),
            connection : each.connection,
        }).collect();

    Ok(parsed)

}

// ======================= ip neigh =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct IpNeighRaw {
    pub dst : String,
    pub dev : String,
    pub lladdr : Option<String>,
    pub state : Vec<String>,
}

pub fn parse_ip_neigh(output: &str) -> Result<Vec<NeighborState>,String> {
    let raw : Vec<IpNeighRaw> = serde_json::from_str(&output).map_err(|e|e.to_string())?;

    // dbg!(&raw);
    let parsed = raw.into_iter().map(|each| NeighborState {
        ip : each.dst,
        interface : each.dev,
        mac : each.lladdr,
        is_reachable: each.state.iter().any(|s| {
                matches!(s.as_str(), "REACHABLE" | "STALE" | "DELAY" | "PROBE" | "PERMANENT")
            }),
    }).collect();

    Ok(parsed)
}