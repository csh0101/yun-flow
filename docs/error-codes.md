# YunFlow Error Codes & Recovery Specification

- **Specification ID**: YUN-SPEC-ERR-001
- **Status**: Standard Track
- **Updated**: 2026-09-12

---

## 1. Overview

YunFlow builds upon standard **JSON-RPC 2.0 error framing** (RFC 4627) while reserving the **`-32000` to `-32099`** range for system-specific, deterministic protocol errors.

Every error response returned by a YunFlow server, edge runtime, or peer agent conforms to the standard payload structure:

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32001,
    "message": "Patch revision conflict",
    "data": {
      "error_name": "PatchRevisionConflict",
      "surface_id": "sre-control-center",
      "current_server_revision": 1045,
      "client_from_revision": 1042,
      "recovery_action": "REQUEST_SNAPSHOT",
      "retryable": false
    }
  },
  "id": "req-102"
}
```

---

## 2. Standard Error Code Space

### 2.1 JSON-RPC 2.0 Reserved Codes

| Code | Standard Name | Meaning in YunFlow |
|---|---|---|
| `-32700` | `ParseError` | Malformed JSON received on the wire transport. |
| `-32600` | `InvalidRequest` | Message does not conform to standard JSON-RPC envelope. |
| `-32601` | `MethodNotFound` | Method not implemented by the target surface, agent, or daemon. |
| `-32602` | `InvalidParams` | Parameter failed Serde or JSON Schema validation. |
| `-32603` | `InternalError` | Unhandled panic or internal control plane failure. |

---

### 2.2 YunFlow Protocol Domain Codes (`-32000` to `-32099`)

#### A. State Projection & Synchronization (`-32000` ~ `-32009`)

| Code | Error Name | Cause | Recovery Strategy |
|---|---|---|---|
| **`-32001`** | `PatchRevisionConflict` | The `from_revision` of an incoming patch does not match the client's current revision. | Client MUST discard patch and emit `yunflow.state.snapshot_request`. |
| **`-32002`** | `StateEpochMismatch` | Server rebooted or event store was recreated, invalidating client epoch. | Client MUST wipe local state cache and re-handshake from scratch. |
| **`-32003`** | `PatchApplicationFailed` | An RFC 6902 patch failed to apply (e.g. array index out of bounds, target field missing). | Client catches error, resets to last known good snapshot, and requests full resync. |
| **`-32004`** | `SurfaceSessionExpired` | The session token or surface lifecycle has terminated in `yund`. | Client triggers re-authentication and re-handshake. |
| **`-32005`** | `StateSchemaValidationFailed` | State reducer produced a state object violating the declared JSON schema. | Server logs critical alert; state mutation rejected; preserves previous valid state. |

#### B. Preflight & Human-in-the-Loop Security (`-32010` ~ `-32019`)

| Code | Error Name | Cause | Recovery Strategy |
|---|---|---|---|
| **`-32010`** | `PreflightRejected` | The human operator clicked "Reject" or denied the high-risk action checkpoint. | Client displays rejection banner; AgentLoop records rejection in `rejected_hypotheses`. |
| **`-32011`** | `PreflightGateTimeout` | Human operator did not approve action within the configured TTL (default: 300s). | Intent is aborted; resources unlocked; AgentLoop notifies operator of timeout. |
| **`-32012`** | `IntentRiskUnderreported` | Agent or UI declared `risk_level: LOW`, but Control Plane policy engine reclassified as `HIGH`. | Intent is intercepted and escalated to Human Preflight modal; execution paused. |
| **`-32013`** | `UnauthorizedIntent` | Surface lacks capability grant (`Principal & Grant`) for the requested intent. | Execution blocked immediately; security event logged to audit journal. |

#### C. A2A Peer Collaboration (`-32020` ~ `-32029`) [Preview]

| Code | Error Name | Cause | Recovery Strategy |
|---|---|---|---|
| **`-32020`** | `A2ADepthExceeded` | Delegation chain exceeded `MAX_A2A_DEPTH` (default: 5). | Recursion terminated; error escalated to root orchestrator agent. |
| **`-32021`** | `A2ACycleDetected` | A child agent dispatched an intent targeting an active ancestor in the causality DAG. | Execution halted immediately; loop broken; root operator alerted. |
| **`-32022`** | `A2ASubscriptionNotFound` | Agent attempted to subscribe to a non-existent peer surface or closed session. | Re-probe catalog to discover available peer agents. |

#### D. Capability Detectors & Edge Execution (`-32030` ~ `-32039`)

| Code | Error Name | Cause | Recovery Strategy |
|---|---|---|---|
| **`-32030`** | `DetectorExecutionFailed` | Micro-Wasm sandbox trapped or probe execution failed. | Fallback to Agentic Skill probe or report capability as `UNAVAILABLE`. |
| **`-32031`** | `DetectorPolicyViolation` | Probe script attempted forbidden mutation command (`rm`, `sudo`, `>`). | Command rejected at parser level; edge node raises security alert. |
| **`-32032`** | `SecretScrubbingTriggered` | Sensitive credential detected in probe output; scrubbed in memory. | Informational warning; sanitized snapshot proceeds to Control Plane. |

---

## 3. Recommended Client Error Handling Pattern (TypeScript)

```typescript
import { YunFlowError, YunFlowErrorCode } from '@yunflow/sdk';

client.on('error', (err: YunFlowError) => {
  switch (err.code) {
    case YunFlowErrorCode.PatchRevisionConflict:
    case YunFlowErrorCode.PatchApplicationFailed:
    case YunFlowErrorCode.StateEpochMismatch:
      console.warn(`[YunFlow] Desynchronization detected (${err.code}), requesting full snapshot.`);
      client.requestSnapshot();
      break;

    case YunFlowErrorCode.PreflightRejected:
      console.info(`[YunFlow] Action was explicitly rejected by operator.`);
      ui.showNotice('Action cancelled by user.', 'info');
      break;

    case YunFlowErrorCode.PreflightGateTimeout:
      ui.showNotice('Action timed out waiting for authorization.', 'warning');
      break;

    default:
      console.error(`[YunFlow] Unhandled protocol error [${err.code}]: ${err.message}`, err.data);
      ui.showErrorModal(err.message);
  }
});
```
