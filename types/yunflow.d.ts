/**
 * YunFlow Protocol (YFP) - TypeScript Definitions
 * Specification Version: 1.0.0
 * https://github.com/csh0101/yun-flow
 */

export namespace YunFlow {
  export type ProtocolVersion = "1.0.0";

  /** Wire JSON-RPC Envelope */
  export interface Envelope<T = unknown> {
    jsonrpc: "2.0";
    id?: string | number | null;
    method?: string;
    params?: T;
    result?: unknown;
    error?: ProtocolError;
  }

  export interface ProtocolError {
    code: number;
    message: string;
    data?: unknown;
  }

  /** Standard Error Codes */
  export enum ErrorCodes {
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
    PreflightRejected = -32001,
    RevisionConflict = -32002,
    SurfaceNotFound = -32003,
    Unauthorized = -32004,
  }

  /** Session Handshake */
  export interface InitializeParams {
    client_version: string;
    protocol_version: ProtocolVersion;
    supported_features: Array<"rfc6902_patch" | "adaptive_cards" | "compression_gzip">;
  }

  export interface SurfaceAttachParams {
    surface_id: string;
    session_id: string;
    last_known_revision?: number;
  }

  /** Downlink: State Projection */
  export interface StateSnapshotParams<TState = Record<string, unknown>> {
    session_id?: string;
    surface_id: string;
    revision: number;
    epoch: string;
    timestamp: number;
    state: TState;
  }

  export interface JsonPatchOperation {
    op: "add" | "remove" | "replace" | "move" | "copy" | "test";
    path: string;
    value?: unknown;
    from?: string;
  }

  export interface StatePatchParams {
    session_id?: string;
    surface_id: string;
    from_revision: number;
    to_revision: number;
    timestamp: number;
    patch: JsonPatchOperation[];
  }

  /** Uplink: Intent Stream */
  export type RiskLevel = "LOW" | "MEDIUM" | "HIGH" | "CRITICAL";

  export interface PreflightCheckpoint {
    risk_level: RiskLevel;
    requires_confirmation: boolean;
    confirmation_message?: string;
  }

  export interface IntentDispatchParams<TPayload = Record<string, unknown>> {
    intent_id: string;
    session_id?: string;
    surface_id: string;
    intent_name: string;
    timestamp: number;
    payload: TPayload;
    checkpoint?: PreflightCheckpoint;
  }

  export interface IntentReceipt {
    intent_id: string;
    status: "QUEUED" | "ACCEPTED" | "REJECTED" | "EXECUTING" | "COMPLETED";
    executed_at?: number;
    error?: string;
  }

  /** Capability Insight & Detectors */
  export type CapabilityStatus = "READY" | "DEGRADED" | "UNAVAILABLE" | "UNAUTHORIZED";

  export interface CapabilitySnapshot {
    capability_kind: string;
    edge_id?: string | null;
    status: CapabilityStatus;
    timestamp: number;
    attributes: Record<string, unknown>;
    available_actions?: string[];
    error?: { code: string; message: string } | null;
  }

  export interface DeclarativeRules {
    env?: string[];
    fs_paths?: string[];
    tcp_ports?: number[];
    sockets?: string[];
    exec?: { command: string[]; expect_exit_code?: number };
  }

  export interface CapabilityDetector {
    id: string;
    target_capability: string;
    kind: "declarative" | "wasm_sandbox" | "agent_skill";
    declarative_rules?: DeclarativeRules;
    wasm_spec?: { artifact_digest: string; entrypoint?: string };
    skill_spec?: { skill_name: string; execution_profile?: string; instructions?: string };
  }

  /** Universal Cognitive Envelope */
  export type UrgencyLevel = "LOW" | "NORMAL" | "HIGH" | "IMMEDIATE";

  export interface ResourceRef {
    provider: string;
    identifier: string;
    scope?: string;
    sub_target?: string;
  }

  export interface CognitiveCapsule {
    summary: string;
    rationale: string;
    evidence?: string[];
    rejected_hypotheses?: string[];
    confidence: number;
    urgency?: UrgencyLevel;
  }

  export interface CognitiveEnvelopeParams<TPayload = Record<string, unknown>> {
    trace: {
      flow_id: string;
      origin: { type: "human" | "agent" | "scheduler"; agent_id: string; session_id?: string };
      causality?: { root_operation_id: string; parent_intent_id?: string; depth: number };
    };
    target: {
      domain: string;
      resource_ref: ResourceRef;
    };
    cognition: CognitiveCapsule;
    suggested_action: {
      intent_name: string;
      parameters: TPayload;
    };
    checkpoint?: PreflightCheckpoint;
  }
}

