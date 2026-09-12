# RFC-0001: YunFlow — A Universal Reactive A2UI (Agent-to-UI) Protocol for Distributed Systems

- **RFC ID**: RFC-0001
- **Title**: YunFlow: Universal Reactive A2UI Protocol for Distributed Agent Systems
- **Author**: Yun Architecture Working Group
- **Status**: Proposed / Standard Track
- **Created**: 2026-09-12
- **Updated**: 2026-09-12 (Realigned with Chat-Centric Architecture)
- **Target Version**: YunFlow v1.0.0

---

## 1. Abstract

YunFlow is an open, reactive **A2UI (Agent-to-UI)** protocol specification designed for modern distributed AI agent systems.

Yun is fundamentally a **Conversational Agent Operating System** (`apps/yun-desktop` / `crates/yun-agent` / `crates/yun-session`). Its primary cognitive and interaction spine is the **Chat Stream** (multi-turn conversation loop, Composer input, and message turns).

While the Chat Stream excels at natural language dialogue, reasoning chains, dynamic clarification (`AskUiState`), and intent guidance, complex operational domains (such as Kubernetes infrastructure, distributed systems, robotics, and code refactoring) also require **rich, high-density interactive spatial viewports** (live topology graphs, resource cards, heatmaps, and actionable runbooks).

**YunFlow formalizes the companion reactive state stream** that connects the conversational AgentLoop to interactive frontend surfaces (Surfaces/Tabs/Canvases) within the same session:
1. **$UI = f(\text{Agent State})$**: Frontend surfaces act as pure functional viewports animated by streaming state projections (RFC 6902 JSON Patch) alongside the active chat.
2. **Intent-Driven Interaction & Chat Feedback Loop**: Human interactions on visual canvases do not bypass the system; they emit structured *Intents* that loop directly back into the Chat Session as turn events subject to security preflight checkpoints.
3. **Decoupled Capability Insight**: Domain capabilities are discovered dynamically via a trinitarian detector framework (Declarative, Micro-Wasm, and Agentic Skill Probes) without hardcoding domain assumptions into core hosts.
4. **Industrial-Grade Managed Persistence**: Business state, events, and transactional inboxes/outboxes are anchored in `yund`'s authoritative, namespaced Event Store to guarantee crash recovery and session replay.

---

## 2. Motivation & The Symbiosis of Conversation and Surface

### 2.1 The Symbiosis: Chat-First Dialogue + Companion Reactive Surfaces

Current AI systems often suffer from two disconnects:
- **Disconnect A: The Raw Text Bottleneck (Chat-Only)**:
  When an operator asks an agent to diagnose a 50-node cluster or inspect complex logs, dumping thousands of lines of raw text/JSON into the chat stream overwhelms the user and degrades readability.
- **Disconnect B: The Static Dashboard Bypass (UI-Only)**:
  Traditional static web consoles (Grafana, K8s Web UI) display rich widgets but completely bypass AI reasoning, diagnostics, and conversational guidance.

Yun solves this through **Symbiosis (双轨共生)**:
```text
  ┌────────────────────────────────────────────────────────────────────────┐
  │                           Yun Desktop (GPUI)                           │
  │                                                                        │
  │  【Track 1: Primary Cognitive Spine】     【Track 2: Companion Surface】│
  │   Conversational Chat Stream               YunFlow Reactive Projection │
  │                                                                        │
  │   User: "Inspect cluster health"                                       │
  │   Agent: "Found memory pressure on         ┌────────────────────────┐  │
  │           worker-node-2. Projecting        │ Cluster Status: WARN   │  │
  │           topology to viewport..."         │ [Node-1: OK]           │  │
  │                                            │ [Node-2: MemoryAlert]  │  │
  │   [Interaction: Approval Dialog]           │   [Safe Drain Button]  │  │
  │   "Execute Safe Drain on node-2?" ◄────────┤  (User clicks button)  │  │
  │                                            └────────────────────────┘  │
  │   Intent loops back into Chat Turn ──────────────────┘                 │
  └────────────────────────────────────────────────────────────────────────┘
```

1. **The Chat Stream is the Master Spine**: It retains conversation context, Chain-of-Thought explanations, user guidance, and audit history.
2. **The YunFlow Surface is the High-Density Canvas**: It provides spatial orientation, visual summaries, and interactive controls.
3. **Everything is Bound to the Session**: The Chat turns and the YunFlow state projections share the same `session_id`. An action in the viewport is seamlessly woven into the conversation history.

### 2.2 Core Philosophy: "Chat as Mind, Viewport as Canvas"

YunFlow establishes the principle:
> **"The conversation is the cognitive heart; the viewport is its dynamic visual canvas. Data flows from observation to cognition, projects to the canvas, and user intents in the canvas loop back into the conversation turn."**

---

## 3. Mental Model & Plane Separation

YunFlow operates across three decoupled planes, firmly anchored by the **Chat Session**:

```mermaid
flowchart TB
    subgraph PresentationPlane["1. Presentation Plane (yun-desktop)"]
        subgraph ChatSpine["Primary Dialog Spine (Chat Stream)"]
            Composer["Composer (User Input & Slash Commands)"]
            MsgList["Message List (Turn Events, Reasoning & Approvals)"]
        end

        subgraph ViewportSurfaces["Companion Viewports (YunFlow Streams)"]
            UI["Plugin Surface (TabsPanel / Sandboxed Webview)<br/>UI = f(State Projection)"]
        end

        UI ==>|YunFlow: Intent Stream (Button Click)| MsgList
    end

    subgraph ControlPlane["2. Control Plane (yund / Cognitive Core)"]
        AgentLoop["AgentLoop (Planner, Reasoner & Turn Engine)"]
        SessionStore["SessionStore & Managed Event Store"]
        WasmState["State Reducer (Projection Controller)"]
        Preflight["Preflight Checkpoint & Security Broker"]
        CapRouter["Heterogeneous Capability Router"]

        Composer -->|user/message| AgentLoop
        AgentLoop -->|assistant/chunk| MsgList
        AgentLoop <--> WasmState
        WasmState <--> SessionStore
        Preflight --> AgentLoop
        AgentLoop --> CapRouter
    end

    subgraph ExecutionMesh["3. Heterogeneous Execution & Capability Mesh"]
        LocalTools["Control-Local Tools (AST / Memory / Calculations)"]
        CloudAPIs["Stateless Cloud & SaaS APIs (GitHub, Datadog, AWS)"]
        MCPServers["External MCP Servers (Postgres, Docker, Files)"]
        EdgeNodes["Edge Hosts (yun-edge: K8s Container, Mac, Bare-Metal)"]

        CapRouter --> LocalTools
        CapRouter --> CloudAPIs
        CapRouter --> MCPServers
        CapRouter --> EdgeNodes
    end

    %% YunFlow Protocols
    WasmState ==>|YunFlow: State Projection Stream (RFC 6902)| UI
    UI ==>|YunFlow: Intent Stream (Action Dispatch)| Preflight
    EdgeNodes -.->|Observation Stream (Telemetry & Events)| AgentLoop
```

### 3.1 The Presentation Plane (`yun-desktop`)
- **Dual-Track Rendering**:
  - **Chat Track**: GPUI-native Markdown message stream, tool execution badges, ask dialogs (`AskUiState`), and preflight approval buttons (`PendingApproval`).
  - **Surface Track**: Sandboxed Webviews or native components in the Tabs panel ([`apps/yun-desktop/src/minke/tabs_panel.rs`](file:///Users/d-robotics/orca/dsh-rs/apps/yun-desktop/src/minke/tabs_panel.rs)) rendering pure projections.
- **Zero Direct Credentials**: Frontend code never holds `kubeconfig` or database secrets; it only receives projected domain state and emits typed intents.

### 3.2 The Control Plane (`yund`)
- The cognitive and authoritative heart of the system.
- Hosts the **AgentLoop**, which drives conversation turns (`SessionTurnRuntime`), reasons over goals, executes tools, and evaluates domain health.
- Manages **Namespaced Controller State** and the append-only **Event Store / SessionStore** to guarantee persistence across client restarts.

### 3.3 The Heterogeneous Capability Mesh
- **AgentLoop is NOT coupled to any single host.**
- Tools are executed wherever appropriate:
  - **Stateless tools** execute in-process inside `yund`.
  - **SaaS / Web tools** execute over standard HTTPS.
  - **Physical/environmental tools** route dynamically to designated **Edge nodes (`yun-edge`)** when local filesystems, PTYs, or in-cluster sockets are strictly required.

---

## 4. YunFlow Protocol Specification

YunFlow operates via two primary streams connecting the Presentation Plane and Control Plane, built upon standard web protocols:

```text
 ┌────────────────────────────────────────────────────────┐
 │                      YunFlow Wire                      │
 ├────────────────────────┬───────────────────────────────┤
 │ 1. Intent Stream       │ Typed Action Spec (Uplink)    │
 ├────────────────────────┼───────────────────────────────┤
 │ 2. State Projection    │ RFC 6902 JSON Patch (Downlink)│
 ├────────────────────────┼───────────────────────────────┤
 │ 3. Wire Transport      │ JSON-RPC 2.0 / SSE / WebSocket│
 └────────────────────────┴───────────────────────────────┘
```

### 4.1 Downlink: State Projection Stream (Agent $\to$ UI)

The AgentLoop processes raw, disparate data sources and synthesizes a high-level **Domain State Projection**.

#### A. Initial State Hydration (Snapshot)
#### A. Initial State Hydration (Snapshot)
Upon client connection or reconnection within a session, the server transmits a full snapshot:
```json
{
  "jsonrpc": "2.0",
  "method": "yunflow.state.snapshot",
  "params": {
    "session_id": "sess-a899dfb3",
    "surface_id": "sre-control-center",
    "revision": 1042,
    "epoch": "epoch-98a72b",
    "timestamp": 1789134000000,
    "state": {
      "cluster_health": "HEALTHY",
      "active_context": "prod-k8s-01 / default",
      "metrics": {
        "nodes_ready": "3/3",
        "pods_running": 42,
        "anomalies": 0
      },
      "anomalies": [],
      "available_runbooks": [
        {
          "id": "rb-node-drain",
          "title": "Safe Node Drain & Quarantine",
          "risk": "HIGH"
        }
      ]
    }
  }
}
```

#### B. Incremental Delta Update (RFC 6902 JSON Patch)
When the Agent detects an anomaly or updates its reasoning, it transmits an incremental RFC 6902 patch rather than resending the entire state:
```json
{
  "jsonrpc": "2.0",
  "method": "yunflow.state.patch",
  "params": {
    "session_id": "sess-a899dfb3",
    "surface_id": "sre-control-center",
    "from_revision": 1042,
    "to_revision": 1043,
    "timestamp": 1789134015000,
    "patch": [
      { "op": "replace", "path": "/cluster_health", "value": "CRITICAL" },
      { "op": "replace", "path": "/metrics/anomalies", "value": 1 },
      {
        "op": "add",
        "path": "/anomalies/0",
        "value": {
          "id": "anom-mem-01",
          "title": "Node worker-node-2 under severe MemoryPressure",
          "impact": "Pod evictions imminent",
          "suggested_runbook": "rb-node-drain"
        }
      }
    ]
  }
}
```

### 4.2 Uplink: Intent Stream (UI $\to$ Agent)

When an operator interacts with the UI (clicking an action button, triggering a runbook, adjusting a configuration parameter), the UI dispatches a typed **Intent**:

```json
{
  "jsonrpc": "2.0",
  "method": "yunflow.intent.dispatch",
  "params": {
    "intent_id": "intent-f81d4fae-7dec",
    "session_id": "sess-a899dfb3",
    "surface_id": "sre-control-center",
    "intent_name": "execute_runbook",
    "timestamp": 1789134020000,
    "payload": {
      "runbook_id": "rb-node-drain",
      "target_node": "worker-node-2",
      "grace_period_seconds": 60
    },
    "checkpoint": {
      "risk_level": "HIGH",
      "requires_confirmation": true,
      "confirmation_message": "Drain and cordon production node worker-node-2?"
    }
  },
  "id": "req-201"
}
```

#### 4.2.1 Preflight Checkpoints & Human-in-the-Loop
1. **Non-destructive Intents**: Executed immediately by the AgentLoop within the active session.
2. **High-Risk Intents**: Suspended by the Control Plane's `PreflightGate`. The client presents an explicit confirmation dialog with an immutable summary of the action. Once authorized, the Agent initiates execution.

#### 4.2.2 The Intent-to-Chat Feedback Loop (Closing the Dialogue)
A foundational design principle of Yun is that **the Chat Session is the single authoritative ledger of all interactions**:
1. **Turn Event Injection**: When an Intent is dispatched from a Surface, `yund`'s `SessionTurnRuntime` captures it as an `interaction/dispatch` event bound to the active `session_id`.
2. **Chat Message Stream Representation**: The Desktop Chat message list ([`apps/yun-desktop/src/message_list.rs`](file:///Users/d-robotics/orca/dsh-rs/apps/yun-desktop/src/message_list.rs)) dynamically renders the action card:
   > *"User triggered `Safe Node Drain` via SRE Control Center for node `worker-node-2`"*
3. **In-Chat Preflight Approval**: If the intent requires human authorization, an interactive approval card ([`apps/yun-desktop/src/approval.rs`](file:///Users/d-robotics/orca/dsh-rs/apps/yun-desktop/src/approval.rs)) is displayed directly in the conversation flow.
4. **Coordinated Resolution**: Upon approval, the AgentLoop executes the tool, streams progress into the Chat message list (`assistant/chunk` / `tool/update`), and publishes the resulting state delta to the Surface via `yunflow.state.patch`.
5. This ensures that an operator can review the complete timeline—both conversational reasoning and visual UI operations—in one coherent conversation history.

### 4.3 The Universal Cognitive Envelope (`CognitiveEnvelope`)

To resolve the fundamental tension between **serialization efficiency (compact JSON)** and **LLM cognitive signal-to-noise ratio (SNR)**, YunFlow formalizes the **Dual-Track Universal Cognitive Envelope**.

Cold JSON diffs alone starve downstream agents or operators of reasoning context ("why was this conclusion reached? What was ruled out?"). Conversely, conversational text overflows with syntactic boilerplate and hallucinations.

```text
 ┌────────────────────────────────────────────────────────┐
 │           YunFlow Universal Cognitive Envelope         │
 ├─────────────────────────┬──────────────────────────────┤
 │ 1. Trace (Causality)    │ flow_id, origin, depth       │
 ├─────────────────────────┼──────────────────────────────┤
 │ 2. Target (Grounding)   │ domain, resource_ref (URI)   │
 ├─────────────────────────┼──────────────────────────────┤
 │ 3. Cognition (High-SNR) │ summary, rationale, evidence,│
 │                         │ rejected_hypotheses, conf    │
 ├─────────────────────────┼──────────────────────────────┤
 │ 4. Suggested Action     │ intent_name, parameters map  │
 ├─────────────────────────┼──────────────────────────────┤
 │ 5. Checkpoint (Safety)  │ risk_level, human_confirm    │
 └─────────────────────────┴──────────────────────────────┘
```

#### The Dual-Track Model
1. **Machine Grounding Track (Deterministic)**: `target.resource_ref` and `suggested_action.parameters` provide Serde-validated, deterministic execution targets with zero hallucination.
2. **Cognitive Rationale Track (High-SNR Context)**:
   - `summary`: One-sentence executive conclusion.
   - `rationale`: Concise causal chain of reasoning.
   - `evidence`: Specific pointers/anchors (Evidence by Reference, not inline dumps).
   - `rejected_hypotheses`: Crucial list of ruled-out possibilities, preventing redundant agent exploration.
   - `confidence`: Calibrated score (0.0 to 1.0).

#### Universality Across Domains
The envelope is strictly domain-agnostic:
- **Software Security**: Code SQL injection (`code.security`).
- **Cloud Infrastructure / SRE**: Node memory pressure (`infra.k8s`).
- **Data Pipelines / BI**: Daily conversion rate anomalies (`biz.analytics`).
- **Embodied Robotics**: Joint motor thermal alerts (`robot.motion`).

```json
{
  "jsonrpc": "2.0",
  "method": "yunflow.cognitive.handoff",
  "params": {
    "trace": { "flow_id": "flow-sec-9812", "origin": { "type": "agent", "agent_id": "agent-sec-auditor" } },
    "target": {
      "domain": "code.security",
      "resource_ref": { "provider": "git", "identifier": "billing/services/order_service.py", "sub_target": "L42-L58" }
    },
    "cognition": {
      "summary": "CWE-89 SQL Injection in order_service.py via raw string formatting",
      "rationale": "User input order_id is interpolated into cursor.execute() without parameterized binding.",
      "evidence": ["git://billing/services/order_service.py#L42-L58: AST BinOp dynamic format string"],
      "rejected_hypotheses": ["Checked database driver: auto-escape is not enabled", "Checked WAF: internal endpoint bypasses Cloudflare"],
      "confidence": 0.98,
      "urgency": "IMMEDIATE"
    },
    "suggested_action": {
      "intent_name": "generate_safe_patch",
      "parameters": { "fix_pattern": "parameterized_query", "target_driver": "psycopg2" }
    },
    "checkpoint": { "risk_level": "MEDIUM", "requires_human_confirmation": false }
  }
}
```

### 4.4 State Synchronization, Reconnection & Catch-Up Protocol

To ensure fault tolerance under network instability, client suspension (e.g. laptop sleep/wake), and daemon restarts, YunFlow defines a deterministic state synchronization and catch-up protocol.

#### 4.4.1 Handshake & Catch-Up Initiation (`yunflow.session.handshake`)
Upon establishing or re-establishing a transport channel (WebSocket / SSE), the client transmits a handshake request declaring its last acknowledged state coordinates:

```json
{
  "jsonrpc": "2.0",
  "method": "yunflow.session.handshake",
  "params": {
    "surface_id": "sre-control-center",
    "session_id": "sess-a899dfb3",
    "client_epoch": "epoch-98a72b",
    "last_revision": 1042,
    "supported_compression": ["zstd", "none"]
  },
  "id": "hs-001"
}
```

#### 4.4.2 Server Catch-Up Decision Matrix
The Control Plane evaluates the client coordinates against its authoritative Event Store and Ring Buffer:

| Scenario | Condition | Server Response Strategy | Action |
|---|---|---|---|
| **A. In-Sync** | `client_epoch == server_epoch` AND `last_revision == current_revision` | `yunflow.session.ack` (`status: "IN_SYNC"`) | Zero data sent; normal event listening resumes. |
| **B. Delta Catch-Up** | `client_epoch == server_epoch` AND `0 < current_revision - last_revision <= MAX_PATCH_BUFFER` (default: 50) | `yunflow.state.catchup` (Sequential Patch Array) | Server streams buffered patches in order; client applies sequentially to reach `current_revision`. |
| **C. Epoch Mismatch** | `client_epoch != server_epoch` (Server rebooted / Store rehydrated) | `yunflow.state.snapshot` (`status: "RESET"`) | Client drops local state entirely and hydrates from full snapshot. |
| **D. Buffer Eviction** | `current_revision - last_revision > MAX_PATCH_BUFFER` | `yunflow.state.snapshot` (`status: "DESYNC_FALLBACK"`) | Incremental catch-up too expensive; server forces full snapshot. |

#### 4.4.3 Delta Catch-Up Frame (`yunflow.state.catchup`)
When within buffer limits, the server streams missing revisions as an atomic batch:
```json
{
  "jsonrpc": "2.0",
  "method": "yunflow.state.catchup",
  "params": {
    "session_id": "sess-a899dfb3",
    "surface_id": "sre-control-center",
    "from_revision": 1042,
    "to_revision": 1045,
    "patches": [
      {
        "revision": 1043,
        "patch": [{ "op": "replace", "path": "/cluster_health", "value": "WARNING" }]
      },
      {
        "revision": 1044,
        "patch": [{ "op": "replace", "path": "/metrics/anomalies", "value": 1 }]
      },
      {
        "revision": 1045,
        "patch": [{ "op": "replace", "path": "/cluster_health", "value": "CRITICAL" }]
      }
    ]
  }
}
```

#### 4.4.4 Client Sync Invariants & Error Recovery
1. **Strict Monotonicity**: A client MUST NOT apply any patch where `from_revision != current_client_revision`.
2. **Atomic Fallback on Error**: If a JSON Patch operation fails (e.g., target array index out of bounds), the client MUST discard partial modifications and immediately dispatch `yunflow.state.sync_error` with code `-32001 (PatchRevisionConflict)` requesting an immediate authoritative snapshot.

---

## 5. Decoupled Capability Insight & The Trinitarian Detector Framework

Because an open platform cannot anticipate what capabilities future plugins will demand (e.g., ROS2 CAN-buses, custom vector databases, obscure cloud SDKs), **capability detection logic is extracted into a standalone, portable abstraction (`CapabilityDetector`)**.

Capability detection is strictly orthogonal to physical execution:
- **Physical Pipeline**: Stage 1: Preflight (Static) $\to$ Stage 2: Connectivity (Network) $\to$ Stage 3: Scope (Permissions).
- **Semantic Detector**: The pluggable artifact defining *what* to inspect.

```text
 ┌───────────────────────────────────────────────────────────────┐
 │               The Trinitarian Detector Spectrum               │
 ├───────────────────┬───────────────────┬───────────────────────┤
 │ 1. Declarative    │ 2. Micro-Wasm     │ 3. Agentic Skill      │
 │ (Rules / Zero-Code)│ (Sandbox VM)     │ (Agent + Bash + Skill)│
 ├───────────────────┼───────────────────┼───────────────────────┤
 │ Simple paths,     │ Proprietary       │ Real-world messy      │
 │ ports, env vars   │ protocol / crypto │ infrastructure, non-  │
 │                   │ handshakes        │ standard configs      │
 │ Latency: < 1ms    │ Latency: < 10ms   │ Latency: ~1-3s        │
 └───────────────────┴───────────────────┴───────────────────────┘
```

### 5.1 Agentic Skill Probe & Anti-Poisoning Guardrails

For complex environments where static rules fail, an ephemeral **Scout Agent** is dispatched using an approved `ProbeSkill` over generic host tools (e.g. `Bash`).

To prevent **Skill Poisoning** or malicious command execution, Edge hosts enforce a strict four-layer sandbox:
1. **AST-Level Read-Only Policy (`ProbeExecutionProfile`)**: Only query commands (`ls`, `cat`, `grep`, `env`, `which`, `ps`, `--version`) are allowed. Mutation commands (`rm`, `sudo`, `chmod`, `dd`, redirects `>`) are rejected at the parser level.
2. **Network Egress Firewall**: Outbound exfiltration of discovered credentials via `curl/wget/nc` is blocked.
3. **Secret Scrubbing**: Private keys, bearer tokens, and passwords discovered in stdout are redacted in-memory (`[REDACTED_SECRET]`) prior to network transit.
4. **Serde Schema Enforcement**: The Agent's final output must strictly conform to the `CapabilitySnapshot` schema; arbitrary prompt injections are discarded.

---

## 6. Managed Industrial Persistence

Plugins do NOT manage private databases or OS file handles. Instead, `yund` provides managed, namespaced persistence:

1. **Namespaced Controller State**:
   - Each plugin declares its state schema.
   - Mutations occur via pure-function transitions: $State_{n+1} = f(State_n, Event)$.
   - States are saved with monotonic revisions via Compare-And-Swap (CAS).
2. **Transactional Inbox / Outbox**:
   - Inbound triggers (cron timers, webhooks, operator clicks) are journaled in `inbox`.
   - Planned Agent actions are journaled in `outbox`.
   - On crash or `SIGKILL`, `yund` recovers pending outbox tasks seamlessly.
3. **Offline Resilience**:
   - Scheduled health checks and autonomous remediation continue in `yund` even when desktop applications are closed.

---

## 7. Relationship with MCP (Model Context Protocol)

YunFlow and Anthropic's MCP operate at different layers and complement each other:

| Feature | Anthropic MCP | YunFlow Protocol |
|---|---|---|
| **Scope** | Agent $\longleftrightarrow$ Tools & Resources | Agent $\longleftrightarrow$ User Interface (A2UI) & Capability Mesh |
| **Topology** | 1:1 Request / Response RPC | Reactive Tri-Stream with Managed Persistence |
| **Presentation** | None (Raw JSON / Text) | Pure Viewport Canvas ($UI = f(\text{State})$) |
| **State Sync** | None | RFC 6902 JSON Patch Differential Streaming |
| **Relationship** | Capability Provider | **YunFlow on MCP**: MCP servers serve as one of YunFlow's capability backends |

---

## 8. Prior Art & Open Standards Heritage

YunFlow is composed entirely of battle-tested open standards:
- **Wire Transport**: JSON-RPC 2.0 (RFC 4627), Server-Sent Events (W3C), WebSockets (RFC 6455).
- **State Synchronization**: JSON Patch (RFC 6902), JSON Merge Patch (RFC 7396).
- **UI Renderers**: W3C HTML5/JS Webview, Microsoft Adaptive Cards.
- **Tool Interoperability**: Anthropic Model Context Protocol (MCP).
