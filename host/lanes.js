// Pseudo-FFI bindings to your Rust crate compiled to WASM or native addon.
// Adjust these imports to match your actual build tooling.
import {
  scheduler_new,
  scheduler_maybe_rotate_turn,
  scheduler_handle_context_event,
  scheduler_try_apply_token,
  scheduler_set_safety_state,
  nav_adapter_new,
  nav_adapter_read_params,
  nav_adapter_propose_tokens,
  nav_adapter_apply_token,
  consent_state_navigation_default,
} from "./ffi/biosphere.js";

const THREE_MINUTES_MS = 180_000;

// Live in-memory state for consent and context.
const state = {
  scheduler: null,
  navAdapter: null,
  consentNavigation: null,
  latestNavigationContext: {
    obstacle_density: 0.0,
    ambient_noise: 0.0,
    crowd_pressure: 0.0,
    requested_heading_deg: 0.0,
  },
  safetyState: "Green", // "Green" | "Yellow" | "Red"
};

// Establish WebSocket to public-space infrastructure.
function connectContextSocket(url) {
  const ws = new WebSocket(url);

  ws.onopen = () => {
    console.log("[lanes] context socket connected");
  };

  ws.onmessage = (event) => {
    try {
      const msg = JSON.parse(event.data);
      handleContextMessage(msg);
    } catch (err) {
      console.warn("[lanes] invalid context message", err);
    }
  };

  ws.onclose = () => {
    console.log("[lanes] context socket closed, retrying in 5s");
    setTimeout(() => connectContextSocket(url), 5000);
  };

  return ws;
}

// Map raw messages into lane suggestions and low-dimensional context summaries.
function handleContextMessage(msg) {
  if (msg.type === "lane_suggestion") {
    const event = {
      kind: msg.kind,          // e.g. "NavigationSuggested"
      issued_by: msg.issuer || "unknown",
      signature_valid: !!msg.signature_valid,
      received_at_ms: Date.now(),
    };
    scheduler_handle_context_event(state.scheduler, event);
  }

  if (msg.type === "env_summary") {
    // These are low-dimensional features, not raw neural data.
    state.latestNavigationContext = {
      obstacle_density: clamp01(msg.obstacle_density ?? 0.0),
      ambient_noise: clamp01(msg.ambient_noise ?? 0.0),
      crowd_pressure: clamp01(msg.crowd_pressure ?? 0.0),
      requested_heading_deg: normalizeDeg(msg.requested_heading_deg ?? 0.0),
    };
  }
}

function clamp01(x) {
  return Math.max(0, Math.min(1, x));
}

function normalizeDeg(d) {
  let v = d % 360;
  if (v < 0) v += 360;
  return v;
}

// Simple consent UI wiring: you can replace this with voice/gesture integration.
export function initConsentUI() {
  const navCheckbox = document.getElementById("consent-navigation");
  const navScopeSelect = document.getElementById("consent-navigation-scope");

  function updateConsent() {
    const enabled = navCheckbox.checked;
    const scope = enabled ? navScopeSelect.value : "None";

    state.consentNavigation = consent_state_navigation_default(scope);
  }

  navCheckbox.addEventListener("change", updateConsent);
  navScopeSelect.addEventListener("change", updateConsent);

  updateConsent();
}

// Initialize scheduler, adapter, and sockets.
export function initLanesControl(contextSocketUrl) {
  state.scheduler = scheduler_new();
  state.navAdapter = nav_adapter_new({ version: 1 });
  state.consentNavigation = consent_state_navigation_default("ConservativeTuning");

  connectContextSocket(contextSocketUrl);

  // Start the 3-minute evolution loop.
  setInterval(runEvolutionTurn, THREE_MINUTES_MS);
}

// One 3-minute evolution turn: rotate window, collect tokens, apply allowed ones.
function runEvolutionTurn() {
  const nowMs = Date.now();

  scheduler_maybe_rotate_turn(state.scheduler, nowMs);

  // SafetyState could be computed from physiological and subjective signals.
  scheduler_set_safety_state(state.scheduler, state.safetyState);

  const context = { ...state.latestNavigationContext };

  // Read current params (for logging / UI only).
  const params = nav_adapter_read_params(state.navAdapter);
  console.log("[lanes] nav params before turn:", params);

  // Ask adapter to propose micro-changes consistent with environment.
  const proposedTokens = nav_adapter_propose_tokens(
    state.navAdapter,
    nowMs,
    context,
    /* max_tokens */ 4
  );

  // Apply tokens only if consent and scheduler allow them.
  for (const token of proposedTokens) {
    const allowed = scheduler_try_apply_token(
      state.scheduler,
      nowMs,
      state.consentNavigation,
      token
    );

    if (!allowed) {
      console.log("[lanes] token rejected by scheduler:", token.delta_label);
      continue;
    }

    const result = nav_adapter_apply_token(state.navAdapter, token);
    if (!result.ok) {
      console.warn("[lanes] failed to apply token:", token.delta_label, result.err);
      continue;
    }

    console.log("[lanes] applied token:", token.delta_label);
  }

  const updatedParams = nav_adapter_read_params(state.navAdapter);
  console.log("[lanes] nav params after turn:", updatedParams);
}
