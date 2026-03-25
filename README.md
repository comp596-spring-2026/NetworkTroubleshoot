# Network Troubleshooter

A cross-platform, GUI-based network troubleshooter that performs layer-by-layer diagnostics and provides clear, human-readable explanations of network issues.

---

## Overview

Modern network troubleshooting tools are fragmented and often require command-line knowledge. This project combines multiple system-level tools into a single application that:

- Runs diagnostics across OSI layers
- Normalizes outputs across platforms
- Provides simple explanations instead of raw logs

---

## Problem

- Non-technical users cannot interpret CLI outputs
- Troubleshooting requires multiple disconnected tools
- Existing tools expose raw data without clear explanations

---

## Solution

This project collects networking data from native OS tools, parses and normalizes the output, and presents:

- Structured results
- Clear diagnostic summaries
- Actionable suggestions with minimal technical noise

---

## Tech Stack

- Rust – core diagnostics engine
- Tauri – desktop GUI
- PowerShell / Native CLI tools – system data collection
- VirtualBox / VMware – cross-platform testing

---

## Target Audience

- Non-technical users troubleshooting connectivity issues
- Junior IT / support engineers
- Networking students

---

## Implementation Overview

### Layer 1 – Physical Layer
**Goal:** Is the network interface up?

- Windows: Get-NetAdapter
- Linux: ip link, nmcli

---

### Layer 2 – Data Link Layer
**Goal:** Is the local network reachable?

- Windows: Get-NetNeighbor
- Linux: ip neigh

---

### Layer 3 – Network Layer
**Goal:** Is IP configuration valid?

- Windows: Get-NetIPConfiguration, Get-NetRoute, Test-Connection
- Linux: ip addr, ip route, ping

---

### Layer 4 – Transport Layer
**Goal:** Can TCP connections be established?

- Windows: Test-NetConnection
- Linux: nc (netcat)

---

### Layer 7 – Application Layer
**Goal:** DNS + HTTP checks

- Windows: Resolve-DnsName, Invoke-WebRequest
- Linux: dig, curl

---

### Path Analysis
**Goal:** Identify where connectivity breaks

- Windows: tracert
- Linux: traceroute

---

## Build & Run Instructions

### Requirements
- Rust (Cargo)
- Node.js + npm
- Tauri prerequisites
- Linux tools: ip, ping, nc, dig, traceroute, nmcli
- Windows: PowerShell, tracert

---

### Clone

git clone 
cd network_troubleshooter

```bash
git clone https://github.com/comp596-spring-2026/NetworkTroubleshoot.git
cd network_troubleshooter
```

---

### Install

npm install

---

### Run

npm run tauri dev

---

### Build

npm run tauri build

---

## Project Structure

Command Execution → Raw Parsing → Normalized Models → Diagnostics Engine → UI

---

## Status

- Parsing layer complete
- Cross-platform support working
- Diagnostics layer in progress

---

## Notes

This tool focuses on clarity over completeness. It is designed to help users understand common networking issues without needing deep technical knowledge.
