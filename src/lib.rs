//! YunFlow Protocol (YFP) Rust Definitions
//! Specification Version: 1.0.0
//! https://github.com/csh0101/yun-flow

use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: &str = "1.0.0";

/// Top-level JSON-RPC 2.0 wire framing envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Envelope<T = serde_json::Value> {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProtocolError>,
}

impl<T> Default for Envelope<T> {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: None,
            params: None,
            result: None,
            error: None,
        }
    }
}

/// Standard protocol error structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Downlink: Full State Snapshot Hydration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateSnapshotParams<TState = serde_json::Value> {
    pub surface_id: String,
    pub revision: u64,
    pub epoch: String,
    pub timestamp: u64,
    pub state: TState,
}

/// RFC 6902 JSON Patch operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonPatchOperation {
    pub op: String, // "add", "remove", "replace", "move", "copy", "test"
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

/// Downlink: Incremental State Patch (RFC 6902).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatePatchParams {
    pub surface_id: String,
    pub from_revision: u64,
    pub to_revision: u64,
    pub timestamp: u64,
    pub patch: Vec<JsonPatchOperation>,
}

/// Uplink: Intent Dispatch from UI Surface or Peer Agent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentDispatchParams<TPayload = serde_json::Value> {
    pub intent_id: String,
    pub surface_id: String,
    pub intent_name: String,
    pub timestamp: u64,
    pub payload: TPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<PreflightCheckpoint>,
    /// [PREVIEW] Origin metadata identifying caller (human operator or peer agent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<IntentOrigin>,
}

/// Preflight security checkpoint for dangerous actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreflightCheckpoint {
    pub risk_level: RiskLevel,
    pub requires_confirmation: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// [PREVIEW] Metadata identifying caller origin in A2A or Human-to-Agent flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentOrigin {
    #[serde(rename = "type")]
    pub origin_type: OriginType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub causality: CausalityTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginType {
    Human,
    Agent,
    Scheduler,
}

/// [PREVIEW] Causality trace for preventing multi-agent loops and enforcing DAG bounds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CausalityTrace {
    pub root_operation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_id: Option<String>,
    pub depth: u32,
}

/// Capability Snapshot emitted by capability detectors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilitySnapshot {
    pub capability_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_id: Option<String>,
    pub status: CapabilityStatus,
    pub timestamp: u64,
    pub attributes: serde_json::Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_actions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CapabilityStatus {
    Ready,
    Degraded,
    Unavailable,
    Unauthorized,
}

/// Universal Cognitive Envelope for high-SNR human and agent handoffs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitiveEnvelopeParams<TPayload = serde_json::Value> {
    pub trace: CognitiveTrace,
    pub target: CognitiveTarget,
    pub cognition: CognitiveCapsule,
    pub suggested_action: SuggestedAction<TPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<PreflightCheckpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveTrace {
    pub flow_id: String,
    pub origin: IntentOriginSummary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causality: Option<CausalityTrace>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentOriginSummary {
    #[serde(rename = "type")]
    pub origin_type: OriginType,
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveTarget {
    pub domain: String,
    pub resource_ref: ResourceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRef {
    pub provider: String,
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitiveCapsule {
    pub summary: String,
    pub rationale: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rejected_hypotheses: Vec<String>,
    pub confidence: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<UrgencyLevel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum UrgencyLevel {
    Low,
    Normal,
    High,
    Immediate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedAction<TPayload = serde_json::Value> {
    pub intent_name: String,
    pub parameters: TPayload,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_serialization() {
        let patch = StatePatchParams {
            surface_id: "sre-control-center".to_string(),
            from_revision: 1,
            to_revision: 2,
            timestamp: 1789134020000,
            patch: vec![JsonPatchOperation {
                op: "replace".to_string(),
                path: "/cluster_health".to_string(),
                value: Some(serde_json::json!("CRITICAL")),
                from: None,
            }],
        };

        let json = serde_json::to_string(&patch).expect("serialization works");
        assert!(json.contains("sre-control-center"));
        assert!(json.contains("CRITICAL"));

        let deserialized: StatePatchParams = serde_json::from_str(&json).expect("deserialization works");
        assert_eq!(deserialized, patch);
    }

    #[test]
    fn test_a2a_intent_with_causality() {
        let intent = IntentDispatchParams {
            intent_id: "intent-a2a-001".to_string(),
            surface_id: "sre-control-center".to_string(),
            intent_name: "remediate_oom".to_string(),
            timestamp: 1789134025000,
            payload: serde_json::json!({ "node": "worker-1" }),
            checkpoint: Some(PreflightCheckpoint {
                risk_level: RiskLevel::High,
                requires_confirmation: true,
                confirmation_message: Some("High risk node drain".to_string()),
            }),
            origin: Some(IntentOrigin {
                origin_type: OriginType::Agent,
                agent_id: Some("agent-diag-01".to_string()),
                session_id: Some("sess-101".to_string()),
                causality: CausalityTrace {
                    root_operation_id: "op-root-1".to_string(),
                    parent_intent_id: None,
                    depth: 1,
                },
            }),
        };

        let json = serde_json::to_string(&intent).expect("serialize A2A intent");
        assert!(json.contains("agent-diag-01"));
        assert!(json.contains("causality"));

        let deserialized: IntentDispatchParams = serde_json::from_str(&json).expect("deserialize A2A intent");
        assert_eq!(deserialized, intent);
    }

    #[test]
    fn test_cognitive_envelope_serialization() {
        let env = CognitiveEnvelopeParams {
            trace: CognitiveTrace {
                flow_id: "flow-sec-01".to_string(),
                origin: IntentOriginSummary {
                    origin_type: OriginType::Agent,
                    agent_id: "agent-sec-01".to_string(),
                    session_id: None,
                },
                causality: Some(CausalityTrace {
                    root_operation_id: "op-01".to_string(),
                    parent_intent_id: None,
                    depth: 1,
                }),
            },
            target: CognitiveTarget {
                domain: "code.security".to_string(),
                resource_ref: ResourceRef {
                    provider: "git".to_string(),
                    identifier: "services/order.py".to_string(),
                    scope: Some("main".to_string()),
                    sub_target: Some("L42-L58".to_string()),
                },
            },
            cognition: CognitiveCapsule {
                summary: "SQL Injection found".to_string(),
                rationale: "Unparameterized input".to_string(),
                evidence: vec!["AST format string".to_string()],
                rejected_hypotheses: vec!["Checked WAF".to_string()],
                confidence: 0.98,
                urgency: Some(UrgencyLevel::Immediate),
            },
            suggested_action: SuggestedAction {
                intent_name: "patch_sql".to_string(),
                parameters: serde_json::json!({ "fix": "param_binding" }),
            },
            checkpoint: None,
        };

        let json = serde_json::to_string(&env).expect("serialize CognitiveEnvelope");
        assert!(json.contains("code.security"));
        assert!(json.contains("SQL Injection found"));

        let deserialized: CognitiveEnvelopeParams = serde_json::from_str(&json).expect("deserialize CognitiveEnvelope");
        assert_eq!(deserialized.target.domain, "code.security");
    }
}

