const { invoke } = window.__TAURI__.core;

const DEFAULT_URL = "google.com";
const DEFAULT_IP = "8.8.8.8";

let currentArgs = null;

function readArgsFromInputsOrDefault() {
  const urlInput =
    document.getElementById("dest-url")?.value.trim() ||
    document.getElementById("target-url")?.value.trim() ||
    "";

  const ipInput =
    document.getElementById("dest-ip-add")?.value.trim() ||
    document.getElementById("target-host")?.value.trim() ||
    "";

  return {
    url: urlInput || DEFAULT_URL,
    ip_address: ipInput || DEFAULT_IP,
  };
}

function getArgs() {
  return currentArgs ?? readArgsFromInputsOrDefault();
}

function toHost(input) {
  const s = (input || "").trim();
  if (!s) return "";

  try {
    const u = s.includes("://") ? new URL(s) : new URL("https://" + s);
    return u.hostname;
  } catch (_) {
    return s.replace(/^https?:\/\//, "").split("/")[0];
  }
}

function normalizeUrl(input) {
  const s = (input || "").trim();
  if (!s) return `https://${DEFAULT_URL}`;
  return s.includes("://") ? s : `https://${s}`;
}

function hideAllPanels() {
  const linuxPanel = document.getElementById("linux");
  const windowsPanel = document.getElementById("windows");

  if (linuxPanel) linuxPanel.style.display = "none";
  if (windowsPanel) windowsPanel.style.display = "none";
}

function showLinuxPanel() {
  const linuxPanel = document.getElementById("linux");
  if (linuxPanel) linuxPanel.style.display = "block";
}

function showWindowsPanel() {
  const windowsPanel = document.getElementById("windows");
  if (windowsPanel) windowsPanel.style.display = "block";
}

function setOutput(element, value) {
  if (element) {
    element.textContent = value;
  }
}

function setupSharedInputs() {
  const submitBtn = document.getElementById("submitBtn");

  submitBtn?.addEventListener("click", () => {
    currentArgs = readArgsFromInputsOrDefault();
    console.log("Saved target:", currentArgs);
  });
}

function setupLinuxHandlers() {
  const ipLinkMsgEl = document.querySelector("#ip-link-output");
  const ipAddrMsgEl = document.querySelector("#ip-addr-output");
  const ipRouteMsgEl = document.querySelector("#ip-route-output");
  const nmcliMsgEl = document.querySelector("#nmcli-output");
  const ipNeighMsgEl = document.querySelector("#ip-neigh-output");
  const ipPingMsgEl = document.querySelector("#ping-output");
  const netcatMsgEl = document.querySelector("#netcat-output");
  const curlMsgEl = document.querySelector("#curl-output");
  const digMsgEl = document.querySelector("#dig-output");
  const traceRouteMsgEl = document.querySelector("#traceroute-output");

  document.querySelector("#run-ip-link")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipLinkMsgEl, "Running...");
    try {
      setOutput(ipLinkMsgEl, await invoke("ip_link"));
    } catch (err) {
      setOutput(ipLinkMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-ip-addr")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipAddrMsgEl, "Running...");
    try {
      setOutput(ipAddrMsgEl, await invoke("ip_addr"));
    } catch (err) {
      setOutput(ipAddrMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-ip-route")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipRouteMsgEl, "Running...");
    try {
      setOutput(ipRouteMsgEl, await invoke("ip_route"));
    } catch (err) {
      setOutput(ipRouteMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-nmcli")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(nmcliMsgEl, "Running...");
    try {
      setOutput(nmcliMsgEl, await invoke("nmcli"));
    } catch (err) {
      setOutput(nmcliMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-ip-neigh")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipNeighMsgEl, "Running...");
    try {
      setOutput(ipNeighMsgEl, await invoke("ip_neigh"));
    } catch (err) {
      setOutput(ipNeighMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-ping")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipPingMsgEl, "Running...");
    try {
      const { ip_address } = getArgs();
      setOutput(ipPingMsgEl, await invoke("ping", { ip: ip_address }));
    } catch (err) {
      setOutput(ipPingMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-netcat")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(netcatMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(netcatMsgEl, await invoke("netcat", { host }));
    } catch (err) {
      setOutput(netcatMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-curl")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(curlMsgEl, "Running...");
    try {
      const { url } = getArgs();
      setOutput(curlMsgEl, await invoke("curl", { url: normalizeUrl(url) }));
    } catch (err) {
      setOutput(curlMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-dig")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(digMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(digMsgEl, await invoke("dig", { host }));
    } catch (err) {
      setOutput(digMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-traceroute")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(traceRouteMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(traceRouteMsgEl, await invoke("traceroute", { host }));
    } catch (err) {
      setOutput(traceRouteMsgEl, `Error: ${err}`);
    }
  });
}

function setupWindowsHandlers() {
  const linkStateMsgEl = document.querySelector("#link-state-output");
  const neighborsMsgEl = document.querySelector("#neighbors-output");
  const ipconfigMsgEl = document.querySelector("#ipconfig-output");
  const routeMsgEl = document.querySelector("#route-output");
  const pingMsgEl = document.querySelector("#win-ping-output");
  const netConnectionMsgEl = document.querySelector("#net-connection-output");
  const dnsMsgEl = document.querySelector("#dns-output");
  const webMsgEl = document.querySelector("#web-output");
  const tracertMsgEl = document.querySelector("#tracert-output");

  document.querySelector("#run-link-state")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(linkStateMsgEl, "Running...");
    try {
      setOutput(linkStateMsgEl, await invoke("link_state"));
    } catch (err) {
      setOutput(linkStateMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-get-neighbors")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(neighborsMsgEl, "Running...");
    try {
      setOutput(neighborsMsgEl, await invoke("get_neighbors"));
    } catch (err) {
      setOutput(neighborsMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-get-ipconfig")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipconfigMsgEl, "Running...");
    try {
      setOutput(ipconfigMsgEl, await invoke("get_ipconfig"));
    } catch (err) {
      setOutput(ipconfigMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-get-route")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(routeMsgEl, "Running...");
    try {
      setOutput(routeMsgEl, await invoke("get_route"));
    } catch (err) {
      setOutput(routeMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-test-connection")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(pingMsgEl, "Running...");
    try {
      const { ip_address } = getArgs();
      setOutput(pingMsgEl, await invoke("test_connection", { host: ip_address }));
    } catch (err) {
      setOutput(pingMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-test-net-connection")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(netConnectionMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(netConnectionMsgEl, await invoke("test_net_connection", { host }));
    } catch (err) {
      setOutput(netConnectionMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-resolve-dns")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(dnsMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(dnsMsgEl, await invoke("resolve_dns_name", { host }));
    } catch (err) {
      setOutput(dnsMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-web-request")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(webMsgEl, "Running...");
    try {
      const { url } = getArgs();
      setOutput(webMsgEl, await invoke("invoke_web_request", { url: normalizeUrl(url) }));
    } catch (err) {
      setOutput(webMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-tracert")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(tracertMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(tracertMsgEl, await invoke("tracert", { host }));
    } catch (err) {
      setOutput(tracertMsgEl, `Error: ${err}`);
    }
  });
}

function normalizeStatus(status) {
  const s = String(status || "").toLowerCase();
  if (s.includes("fail")) return "error";
  if (s.includes("warn")) return "warning";
  return "healthy";
}

function setStatusChip(chipId, cardId, state, label) {
  const chip = document.getElementById(chipId);
  const card = document.getElementById(cardId);
  if (!chip || !card) return;

  chip.className = "status-chip";
  card.classList.remove("status-healthy", "status-warning", "status-error");

  if (state === "error") {
    chip.classList.add("error");
    chip.textContent = label || "Problem";
    card.classList.add("status-error");
  } else if (state === "warning") {
    chip.classList.add("warning");
    chip.textContent = label || "Attention";
    card.classList.add("status-warning");
  } else {
    chip.classList.add("healthy");
    chip.textContent = label || "Healthy";
    card.classList.add("status-healthy");
  }
}

function getExpandedExplanation(diagnostic) {
  const title = String(diagnostic?.title || "").toLowerCase();
  const message = String(diagnostic?.message || "").toLowerCase();
  const status = String(diagnostic?.status || "").toLowerCase();

  if (title.includes("physical connection") && status.includes("fail")) {
    return {
      meaning:
        "The system could not find an active Ethernet or Wi-Fi interface that appears usable.",
      likelyCause:
        "The adapter may be disabled, unplugged, disconnected from Wi-Fi, or not recognized properly by the operating system.",
      nextStep:
        "Check whether Wi-Fi is connected or whether the Ethernet cable is firmly plugged in. If needed, verify that the network adapter is enabled."
    };
  }

  if (title.includes("physical connection") && status.includes("pass")) {
    return {
      meaning:
        "The system found at least one active network interface.",
      likelyCause:
        "Basic physical connectivity appears available at the adapter level.",
      nextStep:
        "If a problem still exists, move to Layer 2 and Layer 3 results to see whether the issue is with local network access, IP configuration, or routing."
    };
  }

  if (title.includes("local network") && status.includes("fail")) {
    return {
      meaning:
        "The device did not detect any reachable neighboring devices on the local network.",
      likelyCause:
        "This can happen if the device is isolated from the network, connected to the wrong network, or unable to communicate properly on the local link.",
      nextStep:
        "Check whether the device is connected to the correct network and whether the router or nearby hosts are reachable."
    };
  }

  if (title.includes("local network") && status.includes("warning")) {
    return {
      meaning:
        "No neighboring devices were detected, but this does not always prove a failure by itself.",
      likelyCause:
        "Some networks may have little recent local traffic, or neighbor information may simply not be populated yet.",
      nextStep:
        "If other layers also show problems, treat this as supporting evidence of a local connectivity issue."
    };
  }

  if (title.includes("local network") && status.includes("pass")) {
    return {
      meaning:
        "The device can see or reach neighboring systems on the local network.",
      likelyCause:
        "Local link communication appears to be functioning.",
      nextStep:
        "If internet access still fails, focus on IP addressing, default gateway, DNS, or remote connectivity."
    };
  }

  if (title.includes("ip address") && message.includes("apipa")) {
    return {
      meaning:
        "The system assigned itself a fallback address in the 169.254.x.x range instead of receiving a normal network address.",
      likelyCause:
        "This usually means DHCP failed, so the device could not get valid configuration from the router or DHCP server.",
      nextStep:
        "Reconnect the network, check whether the router is responding, and try renewing the DHCP lease once that feature is added."
    };
  }

  if (title.includes("ip address") && message.includes("no ip")) {
    return {
      meaning:
        "The device does not currently have a usable IP address on an active network interface.",
      likelyCause:
        "This often happens when DHCP fails, the interface is disconnected, or the connection process did not complete successfully.",
      nextStep:
        "Check the adapter connection first. If the link is active, the next likely area to inspect is DHCP or manual IP configuration."
    };
  }

  if (title.includes("ip address") && message.includes("no usable network interface")) {
    return {
      meaning:
        "The system could not find a non-loopback interface suitable for normal network communication.",
      likelyCause:
        "The network adapter may be disabled, missing, disconnected, or not detected properly.",
      nextStep:
        "Verify that a real network adapter is available and active before troubleshooting higher layers."
    };
  }

  if (title.includes("ip address") && status.includes("pass")) {
    return {
      meaning:
        "A usable IP address is assigned to at least one active interface.",
      likelyCause:
        "Basic network addressing appears to be configured correctly.",
      nextStep:
        "If connectivity still fails, check the default gateway, internet reachability, or DNS results."
    };
  }

  if (title.includes("default gateway") && status.includes("fail")) {
    return {
      meaning:
        "The system does not have a route that directs traffic outside the local network.",
      likelyCause:
        "Gateway information may be missing because DHCP did not complete properly or because routing was configured incorrectly.",
      nextStep:
        "Without a default gateway, internet access usually will not work. Check DHCP or network settings and verify that the router is available."
    };
  }

  if (title.includes("default gateway") && status.includes("pass")) {
    return {
      meaning:
        "A default gateway is present, so the system knows where to send traffic outside the local subnet.",
      likelyCause:
        "Routing appears basically correct at this level.",
      nextStep:
        "If the internet is still unreachable, inspect reachability, DNS, and HTTP results."
    };
  }

  if (title.includes("internet reachability") && status.includes("fail")) {
    return {
      meaning:
        "The device could not reliably reach the target host, or packet loss was severe enough to indicate failure.",
      likelyCause:
        "Possible causes include loss of internet access, an unreachable gateway, unstable Wi-Fi, firewall rules, or a target that is blocked or unavailable.",
      nextStep:
        "Check IP address and default gateway results first. If those are healthy, the issue may be farther upstream or specific to the destination."
    };
  }

  if (title.includes("internet reachability") && status.includes("warning")) {
    return {
      meaning:
        "The target was reachable, but network performance appears degraded.",
      likelyCause:
        "High latency can come from weak Wi-Fi, congestion, VPN overhead, or upstream network problems.",
      nextStep:
        "Try the test again, compare results on another network if possible, and check whether the issue is temporary or persistent."
    };
  }

  if (title.includes("internet reachability") && status.includes("pass")) {
    return {
      meaning:
        "The target was reachable and the measured response looked reasonable.",
      likelyCause:
        "Basic network reachability appears healthy at this stage.",
      nextStep:
        "If a web service still fails, the issue may be at the transport or application layer rather than general connectivity."
    };
  }

  if (title.includes("path trace") && status.includes("warning")) {
    return {
      meaning:
        "The route to the destination was incomplete or only partially visible.",
      likelyCause:
        "Some routers may ignore trace probes, or the destination may be unreachable somewhere along the path.",
      nextStep:
        "Treat this as supporting information rather than proof by itself. Compare it with gateway, ping, and DNS results."
    };
  }

  if (title.includes("path trace") && status.includes("pass")) {
    return {
      meaning:
        "The route to the destination could be traced successfully.",
      likelyCause:
        "Path visibility suggests that traffic can move through the network toward the target.",
      nextStep:
        "If application access still fails, focus on service-level checks such as TCP, DNS, or HTTP."
    };
  }

  if (title.includes("tcp reachability") && status.includes("fail")) {
    return {
      meaning:
        "A direct connection to the destination service could not be established.",
      likelyCause:
        "The host may be unreachable, the service may not be running, the target port may be closed, or a firewall may be blocking access.",
      nextStep:
        "Check whether general connectivity works first. If it does, the failure may be specific to the service or destination port."
    };
  }

  if (title.includes("tcp reachability") && status.includes("pass")) {
    return {
      meaning:
        "A direct service-level connection to the destination succeeded.",
      likelyCause:
        "Transport-level connectivity appears to be working for the tested service.",
      nextStep:
        "If a user-facing problem remains, check higher-level issues such as DNS, HTTP response, authentication, or application-specific errors."
    };
  }

  if (title.includes("dns resolution") && status.includes("fail")) {
    return {
      meaning:
        "The system could not translate the domain name into an IP address.",
      likelyCause:
        "DNS may be unavailable, misconfigured, blocked, or temporarily failing. It is also possible that the domain itself is invalid or unavailable.",
      nextStep:
        "Try another domain to see whether the problem affects all lookups. If general connectivity is healthy, DNS settings may need attention."
    };
  }

  if (title.includes("dns resolution") && status.includes("pass")) {
    return {
      meaning:
        "The domain name was resolved successfully into one or more IP addresses.",
      likelyCause:
        "DNS functionality appears to be working for this lookup.",
      nextStep:
        "If the website still does not load, focus on HTTP response, service availability, or destination-specific issues."
    };
  }

  if (title.includes("http response") && status.includes("fail")) {
    return {
      meaning:
        "The target did not return a usable HTTP response.",
      likelyCause:
        "The site may be down, blocked, unreachable, or failing before an HTTP status could be returned.",
      nextStep:
        "Compare this with DNS and TCP results. If those are healthy, the problem may be specific to the web service itself."
    };
  }

  if (title.includes("http response") && status.includes("warning")) {
    return {
      meaning:
        "The server responded, but the response indicates a problem or unusual condition.",
      likelyCause:
        "A client error may mean the request or URL is invalid, while other unusual responses may indicate service-specific issues.",
      nextStep:
        "Check the exact HTTP status and determine whether the problem is with the server, the request path, or access restrictions."
    };
  }

  if (title.includes("http response") && status.includes("pass")) {
    return {
      meaning:
        "The web service returned a successful or expected HTTP response.",
      likelyCause:
        "Application-layer access appears to be working for this test.",
      nextStep:
        "If the user still reports a problem, it may be intermittent, application-specific, or outside the scope of this basic connectivity test."
    };
  }

  return {
    meaning:
      "This result describes the condition detected for the selected network layer.",
    likelyCause:
      "The exact cause depends on the surrounding results from other layers and whether lower-level checks also failed.",
    nextStep:
      "Use this message together with the overall summary and nearby layer results to decide what to check next."
  };
}

function buildExpandedExplanationText(diagnostic) {
  const extra = getExpandedExplanation(diagnostic);
  if (!extra) return "No additional explanation available.";

  return [
    `What this means: ${extra.meaning}`,
    `Likely cause: ${extra.likelyCause}`,
    `What to check next: ${extra.nextStep}`
  ].join("\n\n");
}

function renderLayerDiagnostics(layerName, diagnostics) {
  const layerMap = {
    LayerOne: {
      output: "layer-1-output",
      chip: "layer-1-chip",
      card: "layer-1-card",
      explanation: "layer-1-explanation"
    },
    LayerTwo: {
      output: "layer-2-output",
      chip: "layer-2-chip",
      card: "layer-2-card",
      explanation: "layer-2-explanation"
    },
    LayerThree: {
      output: "layer-3-output",
      chip: "layer-3-chip",
      card: "layer-3-card",
      explanation: "layer-3-explanation"
    },
    LayerFour: {
      output: "layer-4-output",
      chip: "layer-4-chip",
      card: "layer-4-card",
      explanation: "layer-4-explanation"
    },
    LayerSeven: {
      output: "layer-7-output",
      chip: "layer-7-chip",
      card: "layer-7-card",
      explanation: "layer-7-explanation"
    },
  };

  const meta = layerMap[layerName];
  if (!meta) return "healthy";

  const outputEl = document.getElementById(meta.output);
  const explanationEl = document.getElementById(meta.explanation);

  if (!outputEl) return "healthy";

  if (!diagnostics.length) {
    outputEl.textContent = "No issues reported for this layer.";
    if (explanationEl) {
      explanationEl.textContent = "No further explanation is needed for this layer.";
    }
    setStatusChip(meta.chip, meta.card, "healthy", "Healthy");
    return "healthy";
  }

  let worst = "healthy";
  const outputParts = [];
  const explanationParts = [];

  for (const d of diagnostics) {
    const state = normalizeStatus(d.status);

    if (state === "error") worst = "error";
    else if (state === "warning" && worst !== "error") worst = "warning";

    outputParts.push(`[${d.title}]\n${d.message}`);
    explanationParts.push(`[${d.title}]\n${buildExpandedExplanationText(d)}`);
  }

  outputEl.textContent = outputParts.join("\n\n");

  if (explanationEl) {
    explanationEl.textContent = explanationParts.join("\n\n");
  }

  setStatusChip(
    meta.chip,
    meta.card,
    worst,
    worst === "error" ? "Problem" : worst === "warning" ? "Attention" : "Healthy"
  );

  return worst;
}

function setupFullDiagnosticsHandler() {
  const runBtn = document.getElementById("run-scan-btn");
  if (!runBtn) return;

  runBtn.addEventListener("click", async () => {
    const helper = document.getElementById("scan-helper");
    const summaryText = document.getElementById("summary-text");
    const summaryOutput = document.getElementById("summary-output");
    const nextStepText = document.getElementById("next-step-text");
    const nextStepOutput = document.getElementById("next-step-output");

    currentArgs = readArgsFromInputsOrDefault();

    const { url, ip_address } = getArgs();
    const host = toHost(url) || ip_address;
    const finalUrl = normalizeUrl(url);

    runBtn.disabled = true;

    if (helper) helper.textContent = "Running diagnostics...";
    if (summaryText) summaryText.textContent = "Scan in progress...";
    if (summaryOutput) {
      summaryOutput.textContent = "Collecting results from the diagnostics engine...";
    }

    setStatusChip("overall-status-chip", "summary-card", "warning", "Running");

    ["1", "2", "3", "4", "7"].forEach((n) => {
      const out = document.getElementById(`layer-${n}-output`);
      const explanation = document.getElementById(`layer-${n}-explanation`);

      if (out) out.textContent = "Waiting for scan results...";
      if (explanation) explanation.textContent = "Detailed explanation will appear here after the scan.";

      setStatusChip(`layer-${n}-chip`, `layer-${n}-card`, "warning", "Running");
    });

    if (nextStepText) nextStepText.textContent = "Scan in progress.";
    if (nextStepOutput) nextStepOutput.textContent = "Collecting diagnostics...";

    try {
      const diagnostics = await invoke("run_full_diagnostics", {
        host,
        url: finalUrl,
      });

      const grouped = {
        LayerOne: [],
        LayerTwo: [],
        LayerThree: [],
        LayerFour: [],
        LayerSeven: [],
      };

      for (const d of diagnostics) {
        if (grouped[d.layer]) grouped[d.layer].push(d);
      }

      const states = [
        renderLayerDiagnostics("LayerOne", grouped.LayerOne),
        renderLayerDiagnostics("LayerTwo", grouped.LayerTwo),
        renderLayerDiagnostics("LayerThree", grouped.LayerThree),
        renderLayerDiagnostics("LayerFour", grouped.LayerFour),
        renderLayerDiagnostics("LayerSeven", grouped.LayerSeven),
      ];

      let overall = "healthy";
      if (states.includes("error")) overall = "error";
      else if (states.includes("warning")) overall = "warning";

      setStatusChip(
        "overall-status-chip",
        "summary-card",
        overall,
        overall === "error" ? "Problem" : overall === "warning" ? "Attention" : "Healthy"
      );

      if (overall === "error") {
        if (summaryText) summaryText.textContent = "A network problem was detected.";
        if (summaryOutput) {
          summaryOutput.textContent =
            "At least one OSI layer reported a failure. Review the layer cards below to see where the issue begins and what it likely means.";
        }
      } else if (overall === "warning") {
        if (summaryText) summaryText.textContent = "The network may be partially working.";
        if (summaryOutput) {
          summaryOutput.textContent =
            "At least one OSI layer reported a warning. The network may still function, but performance or reliability may be reduced.";
        }
      } else {
        if (summaryText) summaryText.textContent = "Your network looks healthy.";
        if (summaryOutput) {
          summaryOutput.textContent =
            "No major problems were detected in the tested layers. If a user still experiences issues, the problem may be intermittent or specific to a particular application or destination.";
        }
      }

      const firstBad = diagnostics.find((d) => {
        const s = normalizeStatus(d.status);
        return s === "error" || s === "warning";
      });

      if (!firstBad) {
        if (nextStepText) nextStepText.textContent = "No urgent next step needed.";
        if (nextStepOutput) {
          nextStepOutput.textContent =
            "The scan did not find a clear failure. If a problem still exists, try repeating the scan during the issue or testing a different destination.";
        }
      } else {
        if (nextStepText) nextStepText.textContent = `Focus on ${firstBad.title}.`;
        if (nextStepOutput) {
          nextStepOutput.textContent = [
            firstBad.message,
            "",
            buildExpandedExplanationText(firstBad)
          ].join("\n");
        }
      }

      if (helper) helper.textContent = "Scan complete.";
    } catch (err) {
      console.error("run_full_diagnostics failed:", err);
      setStatusChip("overall-status-chip", "summary-card", "error", "Error");

      if (summaryText) summaryText.textContent = "The scan could not complete.";
      if (summaryOutput) summaryOutput.textContent = String(err);
      if (helper) helper.textContent = "The scan failed.";
    } finally {
      runBtn.disabled = false;
    }
  });
}

window.addEventListener("DOMContentLoaded", async () => {
  setupSharedInputs();
  setupFullDiagnosticsHandler();
  hideAllPanels();

  try {
    const os = await invoke("get_os_type");
    console.log("Detected OS:", os);

    if (os === "linux") {
      showLinuxPanel();
      setupLinuxHandlers();
    } else if (os === "windows") {
      showWindowsPanel();
      setupWindowsHandlers();
    } else {
      showLinuxPanel();
      setupLinuxHandlers();
    }
  } catch (err) {
    console.error("Failed to detect OS:", err);
  }
});