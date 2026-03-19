// parser functions for linux commands

use serde::{Deserialize, Serialize};

use crate::models::{InterfaceStatus,
    InterfaceAddress,
    RouteInfo,
    ConnectionStatus,
    NeighborState,
    TcpStatus,
    ReachabilityStatus};

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

// ======================= netcat ======================= 

#[derive(Debug, Serialize, Deserialize)]
pub struct NetCatRaw {
    pub host : String,
    pub ip : String,
    pub port : String,
    pub protocol : String,
    pub status : String,
}

pub fn parse_netcat(output: &str) -> Result<TcpStatus, String> {
    let data: Vec<&str> = output.split_whitespace().collect();

    if data.len() < 8 {
        return Err(format!("unexpected netcat output: {output}"));
    }

    let raw = NetCatRaw {
        host: data[2].to_string(),
        ip: data[3].trim_matches(|c| c == '(' || c == ')').to_string(),
        port: data[4].to_string(),
        protocol: data[6].trim_matches(|c| c == '[' || c == ']').to_string(),
        status: data[7].trim_end_matches('!').to_string(),
    };


    // dbg!(&raw);

    let parsed = TcpStatus {
        protocol : raw.protocol,
        is_successful : raw.status == "succeeded",
    };

    Ok(parsed)
}


// ======================= ping ======================= 


#[derive(Debug, Serialize, Deserialize)]
pub struct PingRaw {
    pub sent: String,
    pub received: String,
    pub loss: String,
    pub total_time: String,
    pub avg_latency_ms: String,
}

pub fn parse_ping(output: &str) -> Result<ReachabilityStatus, String> {
    // Example stats line:
    // 2 packets transmitted, 2 received, 0% packet loss, time 1001ms
    let stats_line = output
        .lines()
        .find(|line| line.contains("packets transmitted"))
        .ok_or("Could not find ping statistics line")?;

    // Example RTT line:
    // rtt min/avg/max/mdev = 13.801/14.001/14.201/0.200 ms
    let rtt_line = output
        .lines()
        .find(|line| line.contains("min/avg/max"))
        .ok_or("Could not find RTT line")?;

    let stats_parts: Vec<&str> = stats_line.split(',').map(|s| s.trim()).collect();
    if stats_parts.len() < 4 {
        return Err(format!("Unexpected ping statistics format: {stats_line}"));
    }

    let sent = stats_parts[0]
        .split_whitespace()
        .next()
        .ok_or("Could not parse sent packets")?
        .to_string();

    let received = stats_parts[1]
        .split_whitespace()
        .next()
        .ok_or("Could not parse received packets")?
        .to_string();

    let loss = stats_parts[2]
        .split_whitespace()
        .next()
        .ok_or("Could not parse packet loss")?
        .to_string();

    let total_time = stats_parts[3]
        .split_whitespace()
        .nth(1)
        .ok_or("Could not parse total ping time")?
        .to_string();

    // Split on '=' and take right side:
    let rtt_values = rtt_line
        .split('=')
        .nth(1)
        .ok_or("Unexpected RTT format")?
        .trim()
        .trim_end_matches(" ms");

    // values look like: 13.801/14.001/14.201/0.200
    let mut rtt_parts = rtt_values.split('/');

    let _min = rtt_parts.next();
    let avg_latency = rtt_parts
        .next()
        .ok_or("Could not parse average latency")?
        .to_string();

    let raw = PingRaw {
        sent,
        received,
        loss,
        total_time,
        avg_latency_ms: avg_latency,
    };

    let loss_num: usize = raw
        .loss
        .trim_end_matches('%')
        .parse::<usize>()
        .map_err(|e| e.to_string())?;

    let avg_latency_num: f64 = raw
        .avg_latency_ms
        .parse::<f64>()
        .map_err(|e| e.to_string())?;

    let latency_threshold_ms = 100.0;

    let parsed = ReachabilityStatus {
        has_loss: loss_num > 0,
        is_reasonable: avg_latency_num <= latency_threshold_ms,
    };

    Ok(parsed)
}