// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Connector, Constraint } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.36: the connector's local environment, seen as the
 * trace of messages its set-value! and forget-value! exchange.
 * Pending scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_36.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.36 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One line of the trace: a message `what` sent `from` a connector
 * or constraint label `to` one. */
export interface TraceEvent {
  readonly from: string;
  readonly to: string;
  readonly what: "informAboutValue" | "informAboutNoValue" | "setValue";
}

/** Builds the module's connector plus a trace of its messages. */
export function makeTracedConnector(_label: string, _log: TraceEvent[]): Connector {
  throw new PendingSolution();
}

/** The module's adder wearing a label for the trace. */
export function tracedAdder(
  _label: string,
  _a1: Connector,
  _a2: Connector,
  _sum: Connector,
  _log: TraceEvent[],
): Constraint {
  throw new PendingSolution();
}

/** The module's multiplier wearing a label for the trace. */
export function tracedMultiplier(
  _label: string,
  _m1: Connector,
  _m2: Connector,
  _product: Connector,
  _log: TraceEvent[],
): Constraint {
  throw new PendingSolution();
}

/** The module's constant wearing a label for the trace. */
export function tracedConstant(
  _label: string,
  _value: number,
  _connector: Connector,
  _log: TraceEvent[],
): Constraint {
  throw new PendingSolution();
}
