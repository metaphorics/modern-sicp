// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { ProbeEvent, Wire } from "../../packages/ch3/src/03-mutable-data.js";
import {
  halfAdder,
  makeAgenda,
  makeWire,
  probe,
  propagate,
} from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.31: the book's `accept-action-procedure!` runs a newly
 * accepted action once immediately, and the exercise asks why. The
 * answer in this edition's terms: the immediate run is what schedules
 * each device's first response from the values the wires already
 * have. An inverter attached to a zero wire must claim `output := 1`
 * two units later, and that claim is exactly what lets the
 * half-adder's sum rise; without it the gates sit silent until an
 * input changes, and the sum never settles to 1 at all. The
 * demonstration runs the book's half-adder sequence on both wire
 * kinds and pins both logs.
 */

/** A wire whose `addAction` appends without the immediate run: the
 * book's redefined `accept-action-procedure!`, the variant the
 * exercise asks about. */
export const makeWireWithoutInitRun = (): Wire => {
  let signal = false;
  let actions: Array<() => void> = [];
  const callEach = (procedures: Array<() => void>): void => {
    for (const procedure of procedures) {
      procedure();
    }
  };
  return {
    getSignal: () => signal,
    setSignal: (value) => {
      if (signal !== value) {
        signal = value;
        callEach(actions);
      }
    },
    addAction: (action) => {
      actions = [action, ...actions];
    },
  };
};

/** Runs the book's half-adder demo on wires from `makeWire`: build
 * the half-adder with probes on sum and carry, set input-1, and
 * propagate. Returns the probe log. */
export const demonstrateWithInitRun = (): ProbeEvent[] => {
  const agenda = makeAgenda();
  const input1 = makeWire();
  const input2 = makeWire();
  const sum = makeWire();
  const carry = makeWire();
  const events: ProbeEvent[] = [];
  probe("sum", sum, agenda, events);
  probe("carry", carry, agenda, events);
  halfAdder(input1, input2, sum, carry, agenda);
  input1.setSignal(true);
  propagate(agenda);
  return events;
};

/** Runs the same demo on wires from `makeWireWithoutInitRun`. */
export const demonstrateWithoutInitRun = (): ProbeEvent[] => {
  const agenda = makeAgenda();
  const input1 = makeWireWithoutInitRun();
  const input2 = makeWireWithoutInitRun();
  const sum = makeWireWithoutInitRun();
  const carry = makeWireWithoutInitRun();
  const events: ProbeEvent[] = [];
  probe("sum", sum, agenda, events);
  probe("carry", carry, agenda, events);
  halfAdder(input1, input2, sum, carry, agenda);
  input1.setSignal(true);
  propagate(agenda);
  return events;
};
