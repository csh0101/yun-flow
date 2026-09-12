# RFC-0002: YunFlow A2A (Agent-to-Agent) Peer Collaboration [PREVIEW]

- **RFC ID**: RFC-0002
- **Title**: YunFlow A2A: Structural State-Driven Peer Collaboration Protocol
- **Author**: Yun Architecture Working Group
- **Status**: Preview / Experimental (Not for immediate production release)
- **Created**: 2026-09-12
- **Target Version**: YunFlow v1.1.0-preview

---

## 1. Abstract

This RFC specifies the **A2A (Agent-to-Agent)** preview extension for the YunFlow protocol. 

Rather than relying on unstructured, verbose natural-language dialogs (which suffer from high token consumption, latency, and hallucinations), YunFlow A2A leverages the **structural isomorphism** of the existing YunFlow wire format:
- **Downlink**: An agent subscribes to a peer agent's structured **State Projection Stream** (RFC 6902 JSON Patch).
- **Uplink**: An agent delegates tasks by dispatching typed **Intents** with causality coordinates.
- **Safety**: Multi-agent loops are bounded by strict DAG causality limits and human-in-the-loop Preflight Checkpoint escalations.

---

## 2. Motivation & Contrast with Chat-Based Multi-Agent Systems

Existing multi-agent frameworks (e.g., AutoGen, CrewAI) typically facilitate agent interaction via natural language chat logs:

```text
[Flawed Chat-Based A2A]
 Agent A ──"Hey B, here is 500 lines of logs, please diagnose"──► Agent B
 Agent A ◄──"I think it's OOM, here is a 400 word essay..."────── Agent B
 ❌ High token cost for passing unformatted text
 ❌ Loss of structured typing and precision
 ❌ Prone to infinite conversational loops
```

In contrast, **YunFlow A2A** treats peer agents identically to how it treats frontend surfaces, enabling **State-Driven Collaboration**:

```text
[YunFlow Structural A2A]
 Agent A ──RFC 6902 Patch: [{ op: replace, path: /cluster_health, value: CRITICAL }]──► Agent B
 Agent B ──Typed Intent: execute_remediation { runbook_id: rb-drain-safe }────────────► Agent A
 ✅ Zero hallucination in state handoff
 ✅ Minimal token consumption
 ✅ Deterministic Serde deserialization
```

---

## 3. Architecture & Mental Model

```mermaid
flowchart TB
    subgraph AgentA["Agent A: SRE Diagnostician (Cognitive Master)"]
        StateA["Domain State Reducer (Incidents / Health)"]
    end

    subgraph AgentB["Agent B: Remediation Specialist (Execution Worker)"]
        ReasonerB["Remediation Planner"]
    end

    subgraph ControlCore["yund Control Plane"]
        EventStore["Managed Event Store & Outbox"]
        PreflightGate["Preflight & Human Escalation Gate"]
        DAGGuard["Causality & Depth Guard (Max Depth: 5)"]
    end

    %% Flow
    StateA ==>|1. State Projection Stream (RFC 6902)| ReasonerB
    ReasonerB ==>|2. Delegated Intent Stream| DAGGuard
    DAGGuard --> PreflightGate
    PreflightGate -->|3. High Risk Escalation| Human[Human Operator]
    Human -.->|Authorized| EventStore
    PreflightGate -->|4. Low Risk Permitted| EventStore
```

---

## 4. Wire Protocol Extensions (Preview)

### 4.1 Peer Subscription (`yunflow.agent.subscribe`)
An agent registers its interest in another agent's domain state:
```json
{
  "jsonrpc": "2.0",
  "method": "yunflow.agent.subscribe",
  "params": {
    "subscriber_agent_id": "agent-remediation-ops",
    "target_surface_id": "sre-control-center",
    "filter": {
      "severity_threshold": "WARNING",
      "categories": ["infrastructure_alert", "outage"]
    }
  },
  "id": "a2a-sub-001"
}
```

### 4.2 Delegated Intent Dispatch with Causality Tracking
When an agent acts upon observed state, it dispatches an Intent carrying explicit causality metadata:
```json
{
  "jsonrpc": "2.0",
  "method": "yunflow.intent.dispatch",
  "id": "a2a-intent-101",
  "params": {
    "intent_id": "intent-a2a-7729b",
    "surface_id": "sre-control-center",
    "intent_name": "execute_runbook",
    "origin": {
      "type": "agent",
      "agent_id": "agent-remediation-ops",
      "session_id": "sess-a899dfb3",
      "causality": {
        "root_operation_id": "op-root-1002",
        "parent_intent_id": "intent-user-initial",
        "depth": 2
      }
    },
    "payload": {
      "runbook_id": "rb-drain-node-safe",
      "target_node": "worker-node-2"
    },
    "checkpoint": {
      "risk_level": "HIGH",
      "requires_confirmation": true
    }
  }
}
```

---

## 5. The Three Inviolable Architectural Guardrails

To prevent multi-agent autonomy from degrading system stability, three strict guardrails are enforced:

### 5.1 Guardrail 1: Preflight Human Escalation
- Autonomous agents **CANNOT** self-authorize actions with `risk_level: HIGH` or `CRITICAL`.
- High-risk intents emitted by an agent are paused at the `PreflightGate`. A prompt is elevated to the Human Operator's desktop.
- *Exception*: Pre-authorized autonomous grants with strict budget/blast-radius limits (e.g., auto-restarting a stateless pod).

### 5.2 Guardrail 2: Causality DAG & Loop Breaker
- Every A2A interaction must increment the `depth` counter.
- If `depth > MAX_A2A_DEPTH` (default: 5), the system immediately halts the chain with error code `-32010 (A2ADepthExceeded)`.
- Cycle detection rejects any intent referencing an active ancestor `intent_id`.

### 5.3 Guardrail 3: Unified Event Store Single Source of Truth
- No agent-to-agent channel may operate as an unlogged dark channel.
- All state handoffs, intent dispatches, and execution receipts are journaled into `yund`'s SQLite Event Store.

---

## 6. Implementation Staging

1. **Stage 1 (Current / Scope of v1.0.0)**: RFC-0001 (E2A + A2UI) is the production target.
2. **Stage 2 (Preview / Experimental)**: Merge RFC-0002 schemas and Rust types under `feature = "preview-a2a"` for prototype testing.
3. **Stage 3 (Future GA)**: Introduce full autonomous agent peer delegation after comprehensive multi-agent safety verification.
