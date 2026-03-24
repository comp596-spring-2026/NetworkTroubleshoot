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

// ======================= Get-NetAdapter ======================= 

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

// ======================= Get-NetNeighbors ======================= 

#[derive(Debug, Serialize, Deserialize)]
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
pub struct IPEntryRaw {
    pub IPAddress: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
                .and_then(|v| v.into_iter().next())
                .map(|entry| entry.IPAddress),
            ipv6: each
                .IPv6Address
                .and_then(|v| v.into_iter().next())
                .map(|entry| entry.IPAddress),
        })
        .collect();

    Ok(parsed)
}

// ======================= Get-NetRoute =======================

#[derive(Debug, Serialize, Deserialize)]
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
pub struct TestNetConnectionRaw {
    pub ComputerName: String,
    pub RemoteAddress: Option<String>,
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