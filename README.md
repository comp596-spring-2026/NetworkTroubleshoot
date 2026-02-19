# Network Troubleshooter
A Cross-Platform, GUI based network troubleshooter with layer-by-layer fault identification and explainable and easy to understand diagnostics.

## The Problem
- Normal Users cannot interpret logs and outputs.
- The tools are fragmented.
- Existing tools require tinkering with CLI.

## Proposed Solution
Building a structured, cross-platform diagnostics tools that compiles fragmented tools and produces human readable summaries and suggests fixes with minimal technical noise.

## Tech Stack
- Rust (For Cross Compactibility and Minimal / None Runtime Error)
- Powershell Cmdlets and Native CLI tools
- Tauri (For Graphical User Interface)
- VirtualBox / VMWare (For virtualization)
- GNS3 (For Network Simulation)

## Target Audience
- Non-Technical End Users

## Secondary Audiences
- IT Support / Junior Admins
- Networking Students

## Implementation Summary 
### Layer 1 - Physical Layer
#### Goal : Is the NIC up / connected ? Wi-Fi SSID ?
#### Windows :
- Get-NetAdapter
- InterfaceDescription
- Netsh 
#### Linux / MacOS :
- ip link / ifconfig
- ethtool
- nmcli 
- networksetup 

### Layer 2 - Data Link Layer
#### Goal : Is the local network reachable ? (MAC Address / ARP)
#### Windows / Linux / MacOS :
- arp
- Get-NetNeighbor

### Layer 3 - Network Layer
#### Goal : is IP assigned ? Subnet ? Default route ? Gateway ? 
#### Windows : 
- Get-NetIPConfiguration
- Get-NetRoute
- Test-Connection
#### Linux / MacOS :
- ip addr / ifconfig 
- ip route / route
- ping

### Layer 4 - Transport Layer
#### Goal : TCP / UDP reachability ? Port Reachability ?
#### Windows : 
- Test-NetConnection
#### Linux / MacOS :
- nc (netcat)
- curl

### Layer 5 - 6 : Session / Presentaion
No special dedicated checks as it is mostly handled by application layer.

### Layer 7 : Application Layer
#### Goal : Can DNS Resolve ? Can fetch HTTP(s) resource ?, etc
#### Windows :
- Resolve-DnsName
- Invoke-WebRequest
- netsh
#### Linux / MacOS :
- dig
- scutil
- curl
- 'HTTP_PROXY' / 'HTTPS_PROXY' environment variables

## Implementation Checklist
- Phase 0 - Project Skeleton / Requirements
- Phase 1 - Core Collectors (Link State, IP, Route, Neighbor / ARP )
- Phase 2 - Connectivity Checks
- Phase 3 - Diagnostics Engine and Rules
- Phase 4 - GUI
- Phase 5 - Evaluation / Polishing 


