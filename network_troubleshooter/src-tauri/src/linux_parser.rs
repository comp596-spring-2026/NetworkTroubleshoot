// parser functions for linux commands

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

// ======================= dig =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct DigRaw {
    pub query: String,
    pub record_type: String,
    pub status: String,
    pub answers: Vec<String>,
}

#[derive(Debug, PartialEq)]
enum DigSection {
    None,
    Question,
    Answer,
}

pub fn parse_dig(output: &str) -> Result<DnsStatus, String> {
    let mut section = DigSection::None;

    let mut query: Option<String> = None;
    let mut record_type: Option<String> = None;
    let mut status: Option<String> = None;
    let mut answers: Vec<String> = Vec::new();

    for each in output.lines() {
        let trimmed = each.trim();

        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with("status:") {
            let value = trimmed
                .split_once(':')
                .ok_or("could not parse status line")?
                .1
                .trim();

            status = Some(value.to_string());
            continue;
        }

        if trimmed.starts_with("QUESTION_SECTION:") {
            section = DigSection::Question;
            continue;
        }

        if trimmed.starts_with("ANSWER_SECTION:") {
            section = DigSection::Answer;
            continue;
        }

        if trimmed.starts_with("- ") {
            let cleaned = trimmed
                .trim_start_matches("- ")
                .trim_matches(&['\'', '"', ' '][..]);

            let parts: Vec<&str> = cleaned.split_whitespace().collect();

            match section {
                DigSection::Question => {
                    if parts.len() >= 3 {
                        let q = parts[0].trim_end_matches('.');
                        let rtype = parts[2];

                        query = Some(q.to_string());
                        record_type = Some(rtype.to_string());
                    }
                }

                DigSection::Answer => {
                    if parts.len() >= 5 {
                        let answer_type = parts[3];
                        let answer_value = parts[4];

                        if answer_type == "A" || answer_type == "AAAA" {
                            answers.push(answer_value.to_string());
                        }
                    }
                }

                DigSection::None => {}
            }
        }
    }

    let raw = DigRaw {
        query: query.ok_or("could not find query in QUESTION_SECTION")?,
        record_type: record_type.ok_or("could not find record type in QUESTION_SECTION")?,
        status: status.ok_or("could not find status")?,
        answers,
    };

    let parsed = DnsStatus {
    query: raw.query,
    record_type: raw.record_type,
    is_successful: raw.status == "NOERROR" && !raw.answers.is_empty(),
    failure_reason: if raw.status == "NOERROR" && !raw.answers.is_empty() {
        None
    } else {
        Some(raw.status)
    },
    resolved_values: raw.answers,
    };

    Ok(parsed)
}

// ======================= curl =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct CurlRaw {
    pub url: String,
    pub status_code: Option<u16>,
}

pub fn parse_curl(output: &str, url: &str) -> Result<HttpStatus, String> {
    let mut status_code: Option<u16> = None;

    for line in output.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("HTTP/") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();

            if parts.len() < 2 {
                return Err(format!("unexpected HTTP status line: {trimmed}"));
            }

            let code = parts[1]
                .parse::<u16>()
                .map_err(|e| e.to_string())?;

            status_code = Some(code);
            break;
        }
    }

    let raw = CurlRaw {
        url: url.to_string(),
        status_code,
    };

    let parsed = HttpStatus {
    url: raw.url,
    status_code: raw.status_code,
    is_successful: raw.status_code.is_some(),
    };

    Ok(parsed)
}

// ======================= traceroute =======================

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceHopRaw {
    pub hop_number: u8,
    pub host: Option<String>,
    pub ip: Option<String>,
    pub latencies_ms: Vec<f64>,
    pub timed_out: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceRaw {
    pub target: String,
    pub hops: Vec<TraceHopRaw>,
}

pub fn parse_traceroute(output: &str, target: &str) -> Result<TraceStatus, String> {
    let mut raw_hops = Vec::new();

    for line in output.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        // Only parse actual hop lines
        let hop_number = match parts[0].parse::<u8>() {
            Ok(n) => n,
            Err(_) => continue,
        };

        // Timeout hop: "3  * * *"
        if parts.len() >= 2 && parts[1] == "*" {
            raw_hops.push(TraceHopRaw {
                hop_number,
                host: None,
                ip: None,
                latencies_ms: Vec::new(),
                timed_out: true,
            });
            continue;
        }

        let mut host: Option<String> = None;
        let mut ip: Option<String> = None;
        let mut latencies_ms: Vec<f64> = Vec::new();

        // Example:
        // 1  router.local (192.168.1.1)  1.123 ms  1.045 ms  0.980 ms
        // 2  10.0.0.1  8.231 ms  9.101 ms  8.900 ms
        if parts.len() >= 3 && parts[2].starts_with('(') && parts[2].ends_with(')') {
            host = Some(parts[1].to_string());
            ip = Some(parts[2].trim_matches(|c| c == '(' || c == ')').to_string());
        } else if parts.len() >= 2 {
            let second = parts[1];

            if second.chars().all(|c| c.is_ascii_digit() || c == '.' || c == ':') {
                ip = Some(second.to_string());
            } else {
                host = Some(second.to_string());
            }
        }

        for token in &parts {
            if let Ok(ms) = token.parse::<f64>() {
                latencies_ms.push(ms);
            }
        }

        raw_hops.push(TraceHopRaw {
            hop_number,
            host,
            ip,
            latencies_ms,
            timed_out: false,
        });
    }

    let raw = TraceRaw {
        target: target.to_string(),
        hops: raw_hops,
    };

    let destination_reached = raw.hops.last().is_some_and(|last| !last.timed_out);

    let hops = raw.hops.into_iter().map(|hop| TraceHop {
        hop_number: hop.hop_number,
        host: hop.host,
        ip: hop.ip,
        latencies_ms: hop.latencies_ms,
        timed_out: hop.timed_out,
    }).collect();

    Ok(TraceStatus {
        target: raw.target,
        hops,
        destination_reached,
    })
}