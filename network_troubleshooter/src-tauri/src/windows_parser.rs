// parser functions for windows commands

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

// ======================= Get-NetAdapter ======================= 

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
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

// ======================= Get-NetNeighbors ======================= 

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NetNeighborRaw {
    pub IPAddress: String,
    pub InterfaceAlias: String,
    pub LinkLayerAddress: String,
    pub State: u32,
}

pub fn parse_net_neighbor(output: &str) -> Result<Vec<NeighborState>, String> {
    let raw: Vec<NetNeighborRaw> =
        serde_json::from_str(output).map_err(|e| e.to_string())?;

    let parsed = raw
        .into_iter()
        .map(|each| NeighborState {
            ip: each.IPAddress,
            interface: each.InterfaceAlias,
            mac: if each.LinkLayerAddress.trim().is_empty() {
                None
            } else {
                Some(each.LinkLayerAddress)
            },
            is_reachable: matches!(each.State, 5 | 4 | 3 | 2 | 6),
        })
        .collect();

    Ok(parsed)
}

// ======================= Get-NetIPConfiguration =======================

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct IPEntryRaw {
    pub IPAddress: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NetIPConfigurationRaw {
    pub InterfaceAlias: String,
    pub IPv4Address: Option<Vec<IPEntryRaw>>,
    pub IPv6Address: Option<Vec<IPEntryRaw>>,
}

pub fn parse_net_ip_config(output: &str) -> Result<Vec<InterfaceAddress>, String> {
    let raw: Vec<NetIPConfigurationRaw> = if output.trim_start().starts_with('[') {
        serde_json::from_str(output).map_err(|e| e.to_string())?
    } else {
        let single: NetIPConfigurationRaw =
            serde_json::from_str(output).map_err(|e| e.to_string())?;
        vec![single]
    };

    let parsed = raw
        .into_iter()
        .map(|each| InterfaceAddress {
            name: each.InterfaceAlias,
            ipv4: each
                .IPv4Address
                .and_then(|v| v.into_iter().find_map(|entry| entry.IPAddress)),
            ipv6: each
                .IPv6Address
                .and_then(|v| v.into_iter().find_map(|entry| entry.IPAddress)),
        })
        .collect();

    Ok(parsed)
}

// ======================= Get-NetRoute =======================

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NetRouteRaw {
    pub DestinationPrefix: String,
    pub NextHop: String,
    pub InterfaceAlias: String,
    pub RouteMetric: Option<u32>,
}

pub fn parse_net_route(output: &str) -> Result<Vec<RouteInfo>, String> {
    let raw: Vec<NetRouteRaw> = if output.trim_start().starts_with('[') {
        serde_json::from_str(output).map_err(|e| e.to_string())?
    } else {
        let single: NetRouteRaw =
            serde_json::from_str(output).map_err(|e| e.to_string())?;
        vec![single]
    };

    let parsed = raw
        .into_iter()
        .map(|route| RouteInfo {
            is_default: route.DestinationPrefix == "0.0.0.0/0"
                || route.DestinationPrefix == "::/0",
            gateway: if route.NextHop.trim().is_empty()
                || route.NextHop == "0.0.0.0"
                || route.NextHop == "::"
            {
                None
            } else {
                Some(route.NextHop)
            },
            interface: route.InterfaceAlias,
            metric: route.RouteMetric,
        })
        .collect();

    Ok(parsed)
}

// ======================= Test-Connection =======================

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TestConnectionRaw {
    pub StatusCode: u32,
    pub ResponseTime: u32,
}

pub fn parse_test_connection(output: &str) -> Result<ReachabilityStatus, String> {
    let raw: Vec<TestConnectionRaw> = if output.trim_start().starts_with('[') {
        serde_json::from_str(output).map_err(|e| e.to_string())?
    } else {
        let single: TestConnectionRaw =
            serde_json::from_str(output).map_err(|e| e.to_string())?;
        vec![single]
    };

    let total = raw.len();

    let mut success_count = 0;
    let mut total_latency = 0;

    for entry in raw {
        if entry.StatusCode == 0 {
            success_count += 1;
            total_latency += entry.ResponseTime;
        }
    }

    let has_loss = success_count < total;

    let avg_latency = if success_count > 0 {
        total_latency as f64 / success_count as f64
    } else {
        f64::INFINITY
    };

    let latency_threshold_ms = 100.0;

    let is_reasonable = avg_latency <= latency_threshold_ms;

    Ok(ReachabilityStatus {
        has_loss,
        is_reasonable,
    })
}

// ======================= Test-NetConnection =======================

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TestNetConnectionRaw {
    pub ComputerName: String,
    pub RemoteAddress: Option<Value>,
    pub RemotePort: u16,
    pub TcpTestSucceeded: bool,
}

pub fn parse_test_net_connection(output: &str) -> Result<TcpStatus, String> {
    let raw: TestNetConnectionRaw =
        serde_json::from_str(output).map_err(|e| e.to_string())?;

    let protocol = match raw.RemotePort {
        443 => "tcp/https".to_string(),
        80 => "tcp/http".to_string(),
        port => format!("tcp/{port}"),
    };

    Ok(TcpStatus {
        protocol,
        is_successful: raw.TcpTestSucceeded,
    })
}

// ======================= Resolve-DnsName =======================

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ResolveDnsRaw {
    pub Name: String,
    pub IPAddress: Option<String>,
    pub QueryType: u32,
}

pub fn parse_resolve_dns(output: &str) -> Result<DnsStatus, String> {
    let raw: Vec<ResolveDnsRaw> = if output.trim_start().starts_with('[') {
        serde_json::from_str(output).map_err(|e| e.to_string())?
    } else {
        let single: ResolveDnsRaw =
            serde_json::from_str(output).map_err(|e| e.to_string())?;
        vec![single]
    };

    if raw.is_empty() {
        return Ok(DnsStatus {
            query: "".into(),
            record_type: "UNKNOWN".into(),
            resolved_values: vec![],
            is_successful: false,
            failure_reason: Some("No DNS records returned".into()),
        });
    }

    let query = raw[0].Name.clone();

    // Prefer A records first
    let mut answers = Vec::new();
    let mut record_type = "UNKNOWN".to_string();

    for entry in &raw {
        if let Some(ip) = &entry.IPAddress {
            match entry.QueryType {
                1 => {
                    record_type = "A".into();
                    answers.push(ip.clone());
                }
                28 => {
                    if answers.is_empty() {
                        record_type = "AAAA".into();
                        answers.push(ip.clone());
                    }
                }
                _ => {}
            }
        }
    }

    let is_successful = !answers.is_empty();

    Ok(DnsStatus {
        query,
        record_type,
        resolved_values: answers,
        is_successful,
        failure_reason: if is_successful {
            None
        } else {
            Some("DNS resolution failed".into())
        },
    })
}

// ======================= Invoke-WebRequest =======================

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct InvokeWebRequestRaw {
    pub StatusCode: u16,
}

pub fn parse_invoke_web_request(
    output: &str,
    url: &str,
) -> Result<HttpStatus, String> {
    let raw: InvokeWebRequestRaw =
        serde_json::from_str(output).map_err(|e| e.to_string())?;

    Ok(HttpStatus {
        url: url.to_string(),
        status_code: Some(raw.StatusCode),
        is_successful: true,
    })
}

// ======================= tracert =======================

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

pub fn parse_tracert(output: &str, target: &str) -> Result<TraceStatus, String> {
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

        // Parse only hop lines: first token should be hop number
        let hop_number = match parts[0].parse::<u8>() {
            Ok(n) => n,
            Err(_) => continue,
        };

        // Timeout hop:
        //  2     *        *        *     Request timed out.
        if trimmed.contains("Request timed out.") {
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

        // Collect latencies:
        // examples:
        //   12 ms
        //   <1 ms
        let mut i = 0;
        while i < parts.len() {
            let token = parts[i];

            if let Ok(ms) = token.parse::<f64>() {
                if i + 1 < parts.len() && parts[i + 1].eq_ignore_ascii_case("ms") {
                    latencies_ms.push(ms);
                }
            } else if let Some(rest) = token.strip_prefix('<') {
                if let Ok(ms) = rest.parse::<f64>() {
                    if i + 1 < parts.len() && parts[i + 1].eq_ignore_ascii_case("ms") {
                        latencies_ms.push(ms);
                    }
                }
            }

            i += 1;
        }

        // Host/IP usually come after the 3 latency probes.
        // Common forms:
        //  1    <1 ms    <1 ms    <1 ms  router.local [192.168.1.1]
        //  2    12 ms    14 ms    13 ms  10.0.0.1
        //  3    25 ms    24 ms    26 ms  example.com [104.18.27.120]
        //
        // Strategy:
        // - find the first token after the latency section that is not part of "ms"
        // - parse [ip] if present
        // - if bracketed IP exists, preceding tokens form hostname
        // - otherwise, last token may be IP-only

        let mut tail_start = None;
        let mut ms_seen = 0;

        for (idx, token) in parts.iter().enumerate() {
            if token.eq_ignore_ascii_case("ms") {
                ms_seen += 1;
                if ms_seen == 3 && idx + 1 < parts.len() {
                    tail_start = Some(idx + 1);
                    break;
                }
            }
        }

        if let Some(start) = tail_start {
            let tail = &parts[start..];

            // Look for bracketed IP: [1.2.3.4]
            let bracket_ip_index = tail.iter().position(|t| t.starts_with('[') && t.ends_with(']'));

            if let Some(ip_idx) = bracket_ip_index {
                let ip_token = tail[ip_idx];
                ip = Some(ip_token.trim_matches(|c| c == '[' || c == ']').to_string());

                if ip_idx > 0 {
                    let host_tokens = &tail[..ip_idx];
                    let host_joined = host_tokens.join(" ");
                    if !host_joined.is_empty() {
                        host = Some(host_joined);
                    }
                }
            } else if let Some(last) = tail.last() {
                // No [ip], maybe IP-only
                if last.chars().all(|c| c.is_ascii_digit() || c == '.' || c == ':') {
                    ip = Some((*last).to_string());

                    if tail.len() > 1 {
                        let host_joined = tail[..tail.len() - 1].join(" ");
                        if !host_joined.is_empty() {
                            host = Some(host_joined);
                        }
                    }
                } else {
                    // Host only
                    let host_joined = tail.join(" ");
                    if !host_joined.is_empty() {
                        host = Some(host_joined);
                    }
                }
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