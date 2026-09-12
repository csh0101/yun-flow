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

/// Uplink: Intent Dispatch from UI Surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentDispatchParams<TPayload = serde_json::Value> {
    pub intent_id: String,
    pub surface_id: String,
    pub intent_name: String,
    pub timestamp: u64,
    pub payload: TPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<PreflightCheckpoint>,
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
}
