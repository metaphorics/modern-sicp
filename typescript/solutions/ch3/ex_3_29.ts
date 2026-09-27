// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Agenda, ProbeEvent, Wire } from "../../packages/ch3/src/03-mutable-data.js";
import {
  andGate,
  inverter,
  makeAgenda,
  makeWire,
  orGate,
  probe,
  propagate,
} from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.29: the or-gate as a compound device built from and-gates
 * and inverters: `or(a, b) = not(and(not a, not b))`. The exercise's
 * question is the compound gate's delay, so the solution measures both
 * gates on identical schedules: each circuit is built, allowed to
 * settle (the attach-time initialization items of exercise 3.31 fire
 * once), probed only after that transient, and then driven with the
 * same input change. The compound gate's signal path runs through an
 * input inverter, the and-gate, and the output inverter.
 */

/** Builds the compound or-gate: two inverters feeding an and-gate
 * whose output feeds a final inverter. */
export const orGateFromAnd = (a1: Wire, a2: Wire, output: Wire, agenda: Agenda): void => {
  const notA1 = makeWire();
  const notA2 = makeWire();
  const andOut = makeWire();
  inverter(a1, notA1, agenda);
  inverter(a2, notA2, agenda);
  andGate(notA1, notA2, andOut, agenda);
  inverter(andOut, output, agenda);
};

/** One measured circuit: the wires, the agenda, the probe log, and
 * the two calls a scenario needs. */
export interface MeasuredOr {
  readonly agenda: Agenda;
  readonly a1: Wire;
  readonly a2: Wire;
  readonly output: Wire;
  readonly events: ProbeEvent[];
}

/** Builds a settled, probed circuit over the given gate constructor:
 * one settle propagate runs the attach-time initialization transient
 * unprobed, and only then is the probe attached. */
const measureWith = (
  gate: (a1: Wire, a2: Wire, output: Wire, agenda: Agenda) => void,
): MeasuredOr => {
  const agenda = makeAgenda();
  const a1 = makeWire();
  const a2 = makeWire();
  const output = makeWire();
  gate(a1, a2, output, agenda);
  propagate(agenda);
  const events: ProbeEvent[] = [];
  probe("output", output, agenda, events);
  return { agenda, a1, a2, output, events };
};

/** Measures the compound or-gate of this exercise. */
export const measureCompoundOr = (): MeasuredOr => measureWith(orGateFromAnd);

/** Measures the module's primitive or-gate on the same protocol. */
export const measurePrimitiveOr = (): MeasuredOr => measureWith(orGate);

/** Runs one rise of `a1` on a settled circuit and returns the probe
 * log: the attach reading plus the output response. */
export const riseA1 = (circuit: MeasuredOr): ProbeEvent[] => {
  circuit.a1.setSignal(true);
  propagate(circuit.agenda);
  return circuit.events;
};

/** Runs a rise and later fall of `a1` and returns the probe log. */
export const riseAndFallA1 = (circuit: MeasuredOr): ProbeEvent[] => {
  circuit.a1.setSignal(true);
  propagate(circuit.agenda);
  circuit.a1.setSignal(false);
  propagate(circuit.agenda);
  return circuit.events;
};
