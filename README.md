# YunFlow (YFP)

> **A Universal Reactive A2UI (Agent-to-UI) Protocol for Distributed Systems**  
> *让视口流动，而非让代码穿透 (Make the Viewport Flow, Don't Let Code Bypass)*

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![RFC](https://img.shields.io/badge/RFC-0001%20Proposed-green.svg)](rfcs/0001-yunflow-reactive-a2ui-protocol.md)
[![A2UI Standard](https://img.shields.io/badge/Paradigm-A2UI%20(Agent--to--UI)-purple.svg)](#)

---

## 📖 Overview

**YunFlow** is an open protocol specification for **A2UI (Agent-to-UI)** interaction in distributed AI agent architectures.

While Anthropic's **MCP (Model Context Protocol)** defines the standard for *Agent-to-Tool/Resource* calls, and **A2A** defines *Agent-to-Agent* delegation, **YunFlow solves the critical missing link: how an autonomous Agent animates, drives, and collaborates with rich, interactive graphical user interfaces.**

```text
 ┌────────────────────────────────────────────────────────┐
 │                   Frontend Viewport                    │
 │         (SRE Dashboard / Kanban / Custom UI)           │
 └─────────────┬────────────────────────────▲─────────────┘
               │                            │
  [Uplink: Intent Stream]       [Downlink: State Projection Stream]
  - Operator clicks card        - RFC 6902 JSON Patch / Snapshot
  - Triggers remediation        - UI = f(Agent Cognitive State)
  - Preflight authorization     - Flowing, reactive visual updates
               │                            │
 ┌─────────────▼────────────────────────────┴─────────────┐
 │                      AgentLoop                         │
 │     (Cognitive Core / Planner / Managed State Store)   │
 └────────────────────────────┬───────────────────────────┘
                              │
            [Heterogeneous Capability Routing Mesh]
          ┌───────────────────┼───────────────────┐
          ▼                   ▼                   ▼
    Control-Local Tools    Cloud / SaaS APIs     Edge Hosts (yun-edge)
    (Stateless Analysis)   (GitHub / Datadog)    (K8s / Devices / PTY)
```

---

## 🌟 Key Pillars

1. **$UI = f(\text{Agent State})$ (Pure Viewport Shell)**:
   Plugins do not embed raw backend credentials (no `kubeconfig`, no SQL passwords). Frontend canvases are pure projections of the Agent's reasoned domain state.
2. **Intent Stream & Human-in-the-Loop**:
   User interactions emit typed `Intents` rather than executing unchecked CLI commands. High-risk operations pass through **Preflight Checkpoints** for operator confirmation.
3. **Decoupled Capability Insight**:
   The platform has zero prior knowledge of future domain tools. Capabilities are discovered dynamically via a **Trinitarian Detector Framework** (Declarative Rules, Micro-Wasm, and Agentic Skill Probes with AST read-only guardrails).
4. **Managed Industrial Persistence**:
   Business continuity is guaranteed by an authoritative, namespaced Event Store & CAS. Background tasks, health checks, and outboxes persist across desktop restarts.
5. **Built on Open Standards**:
   No proprietary binary framing. YunFlow layers on **JSON-RPC 2.0**, **Server-Sent Events (SSE)**, **RFC 6902 (JSON Patch)**, and **Anthropic MCP**.

---

## 📂 Repository Layout

```text
yun-flow/
├── README.md                               # Project documentation & overview
├── LICENSE                                 # Apache 2.0 License
├── rfcs/                                   # Formal Request for Comments (RFCs)
│   ├── 0001-yunflow-reactive-a2ui-protocol.md  # Core YunFlow Protocol Specification
│   └── template.md                         # Standard RFC template
├── schemas/                                # Wire-level JSON Schemas
│   ├── state-projection.schema.json        # Snapshot & RFC 6902 Patch schema
│   ├── intent-dispatch.schema.json         # Operator intent payload schema
│   ├── capability-snapshot.schema.json     # Edge/Capability evidence schema
│   └── capability-detector.schema.json     # Pluggable detector schema
└── examples/                               # Concrete wire flow examples
    ├── sre-incident-flow/                  # SRE anomaly -> projection -> intent
    └── cloud-board-flow/                   # Pure cloud / zero-edge plugin example
```

---

## 🚀 Quick Example

### 1. State Projection Stream (Agent $\to$ UI)
When an AgentLoop identifies an incident, it streams an incremental patch:
```json
{
  "jsonrpc": "2.0",
  "method": "yunflow.state.patch",
  "params": {
    "surface_id": "sre-control-center",
    "from_revision": 1,
    "to_revision": 2,
    "patch": [
      { "op": "replace", "path": "/cluster_health", "value": "WARNING" },
      {
        "op": "add",
        "path": "/incidents/0",
        "value": {
          "id": "inc-mem-902",
          "title": "Node worker-node-2 MemoryPressure",
          "suggested_runbook_id": "rb-node-drain-safe"
        }
      }
    ]
  }
}
```

### 2. Intent Stream (UI $\to$ Agent)
When the operator clicks "Drain Node", the UI dispatches an Intent:
```json
{
  "jsonrpc": "2.0",
  "method": "yunflow.intent.dispatch",
  "id": "intent-req-001",
  "params": {
    "intent_id": "intent-94aa5c-drain",
    "surface_id": "sre-control-center",
    "intent_name": "execute_runbook",
    "payload": { "runbook_id": "rb-node-drain-safe", "target_node": "worker-node-2" },
    "checkpoint": { "risk_level": "HIGH", "requires_confirmation": true }
  }
}
```

---

## 📜 RFC Process

We welcome community proposals and refinements! See [`rfcs/template.md`](rfcs/template.md) for instructions on submitting a new RFC.

---

## 📄 License

YunFlow is open-sourced under the [Apache License 2.0](LICENSE).
