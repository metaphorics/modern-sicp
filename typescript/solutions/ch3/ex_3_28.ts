// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Agenda, ProbeEvent, Wire } from "../../packages/ch3/src/03-mutable-data.js";
import {
  afterDelay,
  logicalOr,
  makeAgenda,
  makeWire,
  orGateDelay,
  probe,
} from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.28: the or-gate as a primitive function box. The book
 * asks for an `or-gate` constructor similar to `and-gate`; this
 * edition's module already ships one (`orGate`), so the exercise
 * builds its own from the same shape: one action procedure watching
 * both input wires, scheduling the output change one `orGateDelay`
 * after either input changes. The exercise's own logical-or is
 * defined here rather than aliased to the module's.
 */

/** The exercise's own or function: the book's `logical-or` of the
 * or-gate exercise, spelled locally. */
export const logicalOrExercise = (a: boolean, b: boolean): boolean => logicalOr(a, b);

/** Connects `output` to the logical or of the two inputs, the book's
 * primitive `or-gate` written like the module's `andGate`: either
 * input change schedules the output change one `orGateDelay` later. */
export const orGatePrimitive = (a1: Wire, a2: Wire, output: Wire, agenda: Agenda): void => {
  const orActionProcedure = () => {
    afterDelay(
      orGateDelay,
      () => output.setSignal(logicalOrExercise(a1.getSignal(), a2.getSignal())),
      agenda,
    );
  };
  a1.addAction(orActionProcedure);
  a2.addAction(orActionProcedure);
};

/** One wired one-or-gate circuit: the agenda, the probe log the probe
 * appends to, and the three wires, with the probe and the primitive
 * or-gate already attached. The attach reading (time 0, value false)
 * is already in the log. */
export interface OrCircuit {
  readonly agenda: Agenda;
  readonly events: ProbeEvent[];
  readonly a1: Wire;
  readonly a2: Wire;
  readonly output: Wire;
}

/** Builds the book's one-or-gate circuit of this exercise: a probe on
 * the output wire and the primitive or-gate over two fresh inputs. */
export const buildPrimitiveOrCircuit = (): OrCircuit => {
  const agenda = makeAgenda();
  const events: ProbeEvent[] = [];
  const a1 = makeWire();
  const a2 = makeWire();
  const output = makeWire();
  probe("output", output, agenda, events);
  orGatePrimitive(a1, a2, output, agenda);
  return { agenda, events, a1, a2, output };
};
