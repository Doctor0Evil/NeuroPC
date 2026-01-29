// JavaScript orchestrator for ota-neuromorphevolution.
// Packages EvolutionFrames, sends them via JSON-RPC over WebSocket,
// and handles EvolutionDecision callbacks from the Rust inner-ledger.

import WebSocket from "ws";

/**
 * Simple JSON-RPC 2.0 client over WebSocket.
 */
export class LedgerRpcClient {
  constructor({ url, reconnectDelayMs = 3000 }) {
    this.url = url;
    this.reconnectDelayMs = reconnectDelayMs;
    this.ws = null;
    this.pending = new Map();
    this.nextId = 1;
    this.connected = false;
    this._connect();
  }

  _connect() {
    this.ws = new WebSocket(this.url);

    this.ws.on("open", () => {
      this.connected = true;
    });

    this.ws.on("message", (data) => {
      this._handleMessage(data.toString());
    });

    this.ws.on("close", () => {
      this.connected = false;
      setTimeout(() => this._connect(), this.reconnectDelayMs);
    });

    this.ws.on("error", () => {
      this.connected = false;
      this.ws.close();
    });
  }

  _handleMessage(raw) {
    let msg;
    try {
      msg = JSON.parse(raw);
    } catch {
      return;
    }

    if (msg.id && this.pending.has(msg.id)) {
      const { resolve, reject } = this.pending.get(msg.id);
      this.pending.delete(msg.id);

      if (msg.error) {
        reject(msg.error);
      } else {
        resolve(msg.result);
      }
    }
  }

  call(method, params) {
    if (!this.connected) {
      return Promise.reject(new Error("Ledger RPC not connected"));
    }

    const id = this.nextId++;
    const payload = {
      jsonrpc: "2.0",
      id,
      method,
      params,
    };

    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.ws.send(JSON.stringify(payload), (err) => {
        if (err) {
          this.pending.delete(id);
          reject(err);
        }
      });
    });
  }
}

/**
 * Orchestrator layer that:
 * - builds EvolutionFrame objects from host events,
 * - submits them to the Rust ledger,
 * - interprets EvolutionDecision responses,
 * - triggers downstream UI/haptic/rollback actions.
 */
export class EvolutionOrchestrator {
  constructor({ ledgerUrl, hostId, uiSink, hapticSink }) {
    this.hostId = hostId;
    this.rpc = new LedgerRpcClient({ url: ledgerUrl });
    this.uiSink = uiSink;
    this.hapticSink = hapticSink;
  }

  /**
   * Build a typed EvolutionFrame payload for JSON-RPC.
   */
  buildEvolutionFrame({
    plane,
    scope,
    flopBudget,
    nJBudget,
    ecoIntent,
    latencyBand,
    errorBand,
    ecoImpactBand,
    lifeforceBand,
    safetyWave,
    dailyTurnSeq,
  }) {
    return {
      host: this.hostId,
      frame_id: crypto.randomUUID(),
      plane,
      scope,
      cost: {
        flop_budget: flopBudget,
        nJ_budget: nJBudget,
        eco_intent: ecoIntent,
      },
      expected_effect: {
        latency_band: latencyBand,
        error_band: errorBand,
        eco_impact_band: ecoImpactBand,
      },
      guards_snapshot: {
        lifeforce_band: lifeforceBand,
        safety_wave: safetyWave,
        daily_turn_seq: dailyTurnSeq,
      },
    };
  }

  /**
   * Submit a frame and handle the EvolutionDecision.
   */
  async submitFrame(frame) {
    const decision = await this.rpc.call("ledger.applyEvolutionFrame", frame);

    switch (decision.verdict) {
      case "Safe":
        this._onSafe(frame, decision);
        break;
      case "Defer":
        this._onDefer(frame, decision);
        break;
      case "DenyHardStop":
        this._onDenyHardStop(frame, decision);
        break;
      default:
        break;
    }

    return decision;
  }

  /**
   * High-level lane switch helper.
   */
  async suggestLaneSwitch(laneProfile, contextEvent) {
    const frame = this.buildEvolutionFrame({
      plane: laneProfile.plane,
      scope: laneProfile.scope,
      flopBudget: laneProfile.flopBudget,
      nJBudget: laneProfile.nJBudget,
      ecoIntent: laneProfile.ecoIntent,
      latencyBand: laneProfile.latencyBand,
      errorBand: laneProfile.errorBand,
      ecoImpactBand: laneProfile.ecoImpactBand,
      lifeforceBand: contextEvent.lifeforceBand,
      safetyWave: contextEvent.safetyWave,
      dailyTurnSeq: contextEvent.dailyTurnSeq,
    });

    return this.submitFrame(frame);
  }

  _onSafe(frame, decision) {
    if (this.uiSink) {
      this.uiSink.notify({
        type: "evolution_safe",
        frame_id: frame.frame_id,
        applied_deltas: decision.applied_deltas,
      });
    }
    if (this.hapticSink) {
      this.hapticSink.pulse("success");
    }
  }

  _onDefer(frame, decision) {
    if (this.uiSink) {
      this.uiSink.notify({
        type: "evolution_defer",
        frame_id: frame.frame_id,
      });
    }
    if (this.hapticSink) {
      this.hapticSink.pulse("neutral");
    }
  }

  _onDenyHardStop(frame, decision) {
    if (this.uiSink) {
      this.uiSink.notify({
        type: "evolution_denied",
        frame_id: frame.frame_id,
      });
    }
    if (this.hapticSink) {
      this.hapticSink.pulse("alert");
    }
  }
}
