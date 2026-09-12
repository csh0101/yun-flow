# YunFlow Plugin Frontend Developer Quickstart (5-Minute Guide)

Welcome to YunFlow! If you know how to write standard web applications (React, Vue, Svelte, or Vanilla JS), you already know 90% of what you need.

---

## 1. The Core Mental Model in 30 Seconds

```text
  ┌────────────────────────────────────────────────────────┐
  │                 Your Plugin Viewport                   │
  │                                                        │
  │     1. Reactive State Stream (Inbound)                 │
  │        Agent ──[State Projections]──► UI Render        │
  │                                                        │
  │     2. Intent Dispatch Stream (Outbound)               │
  │        User Click ──[Typed Intent]──► Control Plane    │
  └────────────────────────────────────────────────────────┘
```

1. **Zero Hardcoded Credentials**: Your frontend never holds `kubeconfig`, SSH keys, or DB tokens.
2. **Pure State Projection**: You never fetch REST endpoints. The AgentLoop calculates the domain state and pushes it directly into your viewport via **Snapshots** and **RFC 6902 Patches**.
3. **Intent-Driven**: When users click buttons, you dispatch an **Intent** instead of executing the action yourself. The Control Plane verifies permissions and handles execution safely.

---

## 2. Minimal Vanilla JS Example (< 30 Lines)

Inside your plugin's `index.html`:

```html
<!DOCTYPE html>
<html>
<head>
  <title>My First YunFlow Plugin</title>
</head>
<body>
  <h1 id="status">Connecting...</h1>
  <div id="metrics"></div>
  <button id="btn-remediate" style="display:none;">Auto Remediate</button>

  <!-- Import YunFlow Client SDK -->
  <script type="module">
    import { createYunFlowClient } from 'https://esm.sh/@yunflow/sdk';

    const client = createYunFlowClient({
      surfaceId: 'my-plugin-surface'
    });

    // 1. Listen for reactive state updates (auto-handles Snapshots & JSON Patches)
    client.onStateChange((state) => {
      document.getElementById('status').innerText = `Cluster: ${state.cluster_health}`;
      document.getElementById('metrics').innerText = `Anomalies: ${state.metrics?.anomalies ?? 0}`;
      
      const btn = document.getElementById('btn-remediate');
      btn.style.display = state.metrics?.anomalies > 0 ? 'block' : 'none';
    });

    // 2. Dispatch user intent when clicking action buttons
    document.getElementById('btn-remediate').addEventListener('click', async () => {
      await client.dispatchIntent('remediate_anomaly', {
        strategy: 'drain_node',
        grace_period: 30
      });
    });
  </script>
</body>
</html>
```

---

## 3. React Integration (`useYunFlowState` Hook)

For React applications, use the declarative `useYunFlowState` hook:

```tsx
import React from 'react';
import { useYunFlowState, useYunFlowDispatch } from '@yunflow/react';

interface SreState {
  cluster_health: 'HEALTHY' | 'WARNING' | 'CRITICAL';
  metrics: { nodes_ready: string; anomalies: number };
}

export function SreControlCenter() {
  const { state, connected, revision } = useYunFlowState<SreState>('sre-control-center');
  const dispatch = useYunFlowDispatch('sre-control-center');

  if (!connected) return <div>Connecting to Agent Control Plane...</div>;

  const handleDrain = () => {
    dispatch({
      intent_name: 'drain_node',
      payload: { target_node: 'worker-2' },
      checkpoint: {
        risk_level: 'HIGH',
        requires_confirmation: true,
        confirmation_message: 'Are you sure you want to drain worker-2?'
      }
    });
  };

  return (
    <div className={`dashboard-card status-${state.cluster_health.toLowerCase()}`}>
      <h2>Status: {state.cluster_health} (rev: {revision})</h2>
      <p>Nodes: {state.metrics.nodes_ready}</p>
      
      {state.metrics.anomalies > 0 && (
        <button onClick={handleDrain} className="btn-danger">
          Execute Safe Node Drain
        </button>
      )}
    </div>
  );
}
```

---

## 4. Svelte Integration (`yunflowStore`)

```svelte
<script lang="ts">
  import { createYunFlowStore } from '@yunflow/svelte';

  const sreStore = createYunFlowStore('sre-control-center');

  function handleFix() {
    sreStore.dispatchIntent('auto_fix', { target: 'database' });
  }
</script>

{#if $sreStore.loading}
  <p>Synchronizing state with AgentLoop...</p>
{:else}
  <div class="panel">
    <h1>Cluster Status: {$sreStore.state.cluster_health}</h1>
    <button on:click={handleFix}>Trigger Autonomous Fix</button>
  </div>
{/if}
```

---

## 5. Preflight Checkpoints & Human-in-the-Loop Dialogs

If an intent is marked with `risk_level: "HIGH"` or `requires_confirmation: true`, the SDK automatically intercepts the response:

```typescript
try {
  await client.dispatchIntent('drain_node', { target_node: 'worker-node-2' });
  ui.toast('Runbook initiated successfully!');
} catch (err) {
  if (err.code === -32010) {
    ui.toast('Action was rejected by the operator.', 'info');
  } else if (err.code === -32011) {
    ui.toast('Approval request timed out.', 'warning');
  }
}
```

---

## 6. Testing Your Plugin Locally

You do not need a full Kubernetes or cloud backend to test your plugin UI:

1. **Mock Server**: Use the built-in mock runner:
   ```bash
   npx @yunflow/cli mock --state ./mock-state.json --port 8080
   ```
2. Open your browser at `http://localhost:8080`.
3. In your terminal, type:
   ```bash
   patch /cluster_health "CRITICAL"
   ```
   Watch your UI dynamically update in real time!
