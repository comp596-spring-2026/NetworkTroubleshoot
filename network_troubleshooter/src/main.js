const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", () => {
  const ipLinkMsgEl = document.querySelector("#ip-link-output");
  const nmcliMsgEl = document.querySelector("#nmcli-output");
  const ipNeighMsgEl = document.querySelector("#ip-neigh-output");
  const ipPingMsgEl = document.querySelector("#ping-output");
  const netcatMsgEl = document.querySelector("#netcat-output");
  const curlMsgEl = document.querySelector("#curl-output");
  const digMsgEl = document.querySelector("#dig-output");

  document.querySelector("#run-ip-link").addEventListener("submit", async (e) => {
    e.preventDefault();
    ipLinkMsgEl.textContent = "Running...";
    try {
      ipLinkMsgEl.textContent = await invoke("ip_link");
    } catch (err) {
      ipLinkMsgEl.textContent = `Error: ${err}`;
    }
  });

  document.querySelector("#run-nmcli").addEventListener("submit", async (e) => {
    e.preventDefault();
    nmcliMsgEl.textContent = "Running...";
    try {
      nmcliMsgEl.textContent = await invoke("nmcli");
    } catch (err) {
      nmcliMsgEl.textContent = `Error: ${err}`;
    }
  });

  document.querySelector("#run-ip-neigh").addEventListener("submit", async (e) => {
    e.preventDefault();
    ipNeighMsgEl.textContent = "Running...";
    try {
      ipNeighMsgEl.textContent = await invoke("ip_neigh");
    } catch (err) {
      ipNeighMsgEl.textContent = `Error: ${err}`;
    }
  });

  document.querySelector("#run-ping").addEventListener("submit", async (e) => {
    e.preventDefault();
    ipPingMsgEl.textContent = "Running...";
    try {
      ipPingMsgEl.textContent = await invoke("ping");
    } catch (err) {
      ipPingMsgEl.textContent = `Error: ${err}`;
    }
  });

    document.querySelector("#run-netcat").addEventListener("submit", async (e) => {
    e.preventDefault();
    netcatMsgEl.textContent = "Running...";
    try {
      netcatMsgEl.textContent = await invoke("netcat");
    } catch (err) {
      netcatMsgEl.textContent = `Error: ${err}`;
    }
  });

    document.querySelector("#run-curl").addEventListener("submit", async (e) => {
    e.preventDefault();
    curlMsgEl.textContent = "Running...";
    try {
      curlMsgEl.textContent = await invoke("curl");
    } catch (err) {
      curlMsgEl.textContent = `Error: ${err}`;
    }
  });

    document.querySelector("#run-dig").addEventListener("submit", async (e) => {
    e.preventDefault();
    digMsgEl.textContent = "Running...";
    try {
      digMsgEl.textContent = await invoke("dig");
    } catch (err) {
      digMsgEl.textContent = `Error: ${err}`;
    }
  });

});