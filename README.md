# YunFlow (YFP)

> **A Universal Reactive Flow Protocol for Distributed Agent Systems**  
> *连接环境、智能中枢与交互视口的全闭环流动协议 (E2A + A2UI + A2E + A2A Preview)*

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![RFC-0001](https://img.shields.io/badge/RFC--0001-Core%20Reactive%20Flow-green.svg)](rfcs/0001-yunflow-reactive-a2ui-protocol.md)
[![RFC-0002](https://img.shields.io/badge/RFC--0002%20[PREVIEW]-A2A%20Peer%20Collaboration-orange.svg)](rfcs/0002-a2a-peer-collaboration-preview.md)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-blue.svg)](src/lib.rs)

---

## 📖 Overview

**YunFlow** is an open, distributed reactive protocol specification connecting **Physical Environments (Edge)**, **Cognitive Cores (AgentLoop)**, **Interactive Viewports (UI Surfaces)**, and **Autonomous Agent Peers (A2A)**.

While Anthropic's **MCP (Model Context Protocol)** standardizes *Agent-to-Tool* RPCs, **YunFlow solves the complete reactive nervous system**:
1. **E2A (Environment-to-Agent)**: Continuous observation, telemetry, and dynamic capability insight from heterogeneous edges.
2. **A2UI (Agent-to-UI)**: Driving interactive frontend canvases as pure reactive projections ($UI = f(\text{State})$) via RFC 6902 JSON Patch.
3. **A2E (Agent-to-Execution)**: Controlled action dispatch with preflight safety checkpoints and immutable effect journaling.
4. **A2A (Agent-to-Agent, Preview)**: Structured, zero-hallucination peer collaboration via shared state projections and causality DAGs.

```text
                                 ┌─────────────────────────────────┐
                                 │   Yun Desktop (Presentation)   │
                                 └───────────────▲─────────────────┘
                                                 │
                                 【1. A2UI 面向人类的交互界面】
                                 - 状态投影流 (State Projection)
                                 - 意图触发流 (Intent Stream)
                                                 │
 ┌───────────────────────┐       ┌───────────────▼─────────────────┐       ┌───────────────────────┐
 │ Peer Agent (Specialist)│◄─────►│    yund (Control / AgentLoop)   │◄─────►│  Peer Agent (Auditor) │
 └───────────────────────┘       │       【分布式中枢大脑】         │       └───────────────────────┘
   【4. A2A (Preview)】          │   - 状态机计算与因果排序 (CAS)    │         【4. A2A (Preview)】
   - 对等状态投影订阅            │   - 事务信箱与持久化 (EventStore)│         - 委托意图与深度熔断
   - 结构化委托意图              └───────▲─────────────────┬───────┘
                                         │                 │
              【2. E2A 感觉神经 (环境感知流)】               【3. A2E 运动神经 (动作与执行流)】
              - 连续遥测与故障上报 (Observation)             - 异构能力动态路由 (Capability Routing)
              - 三模自适应环境嗅探 (Dynamic Probe)           - 强安全预检门禁 (Preflight Checkpoint)
              - 环境变化副作用收集 (Effect Journal)          - 物理环境落地执行 (Edge / Tools)
                                         │                 │
                                 ┌───────┴─────────────────▼───────┐
                                 │      Heterogeneous Substrates   │
                                 │     (Edge / Cloud / Containers) │
                                 └─────────────────────────────────┘
```

---

## 🌟 Key Pillars

1. **Reactive State Projections (RFC 6902 JSON Patch)**:
   State updates stream down as compact JSON Patch operations, delivering fluid, sub-millisecond viewport reactivity without resending heavy snapshots.
2. **Intent Stream & Human-in-the-Loop**:
   User interactions emit typed `Intents`. High-risk operations pause at **Preflight Checkpoints** for operator authorization.
3. **Decoupled Capability Insight**:
   Zero prior assumptions about future plugins. Capabilities are discovered dynamically via a **Trinitarian Detector Framework** (Declarative Rules, Micro-Wasm, and Agentic Skill Probes with AST read-only guardrails).
4. **Managed Industrial Persistence**:
   Business continuity is guaranteed by `yund`'s authoritative, namespaced Event Store & CAS. Background tasks persist across client restarts.
5. **[PREVIEW] Structural A2A Peer Collaboration**:
   Peer agents subscribe to state streams and delegate sub-intents with strict causality tracing and depth bounding, eliminating token-heavy conversational chat loops.

---

## 📂 Repository Layout

```text
yun-flow/
├── README.md                               # Project overview and architecture
├── LICENSE                                 # Apache 2.0 License
├── Cargo.toml & src/lib.rs                 # Production Rust Serde implementation
├── rfcs/                                   # Formal Request for Comments (RFCs)
│   ├── 0001-yunflow-reactive-a2ui-protocol.md  # Core Reactive Flow (E2A + A2UI + A2E)
│   ├── 0002-a2a-peer-collaboration-preview.md  # [PREVIEW] Structural A2A Collaboration
│   └── template.md                         # Standard RFC template
├── schemas/                                # Wire-level JSON Schemas
│   ├── envelope.schema.json                # JSON-RPC 2.0 framing
│   ├── session-handshake.schema.json       # Session negotiation & attach
│   ├── state-projection.schema.json        # Snapshot & RFC 6902 Patch schema
│   ├── intent-dispatch.schema.json         # Operator & Agent intent payload schema
│   ├── capability-snapshot.schema.json     # Edge/Capability evidence schema
│   ├── capability-detector.schema.json     # Pluggable detector schema
│   └── agent-delegation.schema.json        # [PREVIEW] A2A peer delegation schema
├── types/yunflow.d.ts                       # Frontend TypeScript type declarations
└── examples/                               # Concrete wire flow examples
    ├── sre-incident-flow/                  # SRE anomaly -> projection -> intent
    └── cloud-board-flow/                   # Pure cloud / zero-edge plugin example
```

---

## 📜 RFC Process

- **[RFC-0001: Core Reactive Flow Protocol](rfcs/0001-yunflow-reactive-a2ui-protocol.md)** (Status: **Proposed**)
- **[RFC-0002: A2A Peer Collaboration](rfcs/0002-a2a-peer-collaboration-preview.md)** (Status: **Preview**)

---

## 📄 License

YunFlow is open-sourced under the [Apache License 2.0](LICENSE).
