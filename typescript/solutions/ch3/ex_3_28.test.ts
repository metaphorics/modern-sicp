// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { propagate } from "../../packages/ch3/src/03-mutable-data.js";

import { buildPrimitiveOrCircuit, logicalOrExercise } from "./ex_3_28.js";

describe("exercise 3.28: the or-gate as a primitive function box", () => {
  it("reports the output change one orGateDelay after an input rises", () => {
    const circuit = buildPrimitiveOrCircuit();
    circuit.a1.setSignal(true);
    expect(circuit.events).toEqual([{ time: 0, name: "output", value: false }]);
    propagate(circuit.agenda);
    expect(circuit.events).toEqual([
      { time: 0, name: "output", value: false },
      { time: 5, name: "output", value: true },
    ]);
  });

  it("answers false and stays silent when both inputs stay false", () => {
    const circuit = buildPrimitiveOrCircuit();
    propagate(circuit.agenda);
    expect(circuit.events).toEqual([{ time: 0, name: "output", value: false }]);
    expect(circuit.output.getSignal()).toBe(false);
  });

  it("fires once even when both inputs rise in the same propagate", () => {
    const circuit = buildPrimitiveOrCircuit();
    circuit.a1.setSignal(true);
    circuit.a2.setSignal(true);
    propagate(circuit.agenda);
    expect(circuit.events).toEqual([
      { time: 0, name: "output", value: false },
      { time: 5, name: "output", value: true },
    ]);
  });

  it("a falling input also propagates, back to false at time 5", () => {
    const circuit = buildPrimitiveOrCircuit();
    circuit.a1.setSignal(true);
    propagate(circuit.agenda);
    circuit.a1.setSignal(false);
    propagate(circuit.agenda);
    expect(circuit.events).toEqual([
      { time: 0, name: "output", value: false },
      { time: 5, name: "output", value: true },
      { time: 10, name: "output", value: false },
    ]);
  });

  it("the exercise's logical-or is the truth table of the gates", () => {
    expect(logicalOrExercise(false, false)).toBe(false);
    expect(logicalOrExercise(false, true)).toBe(true);
    expect(logicalOrExercise(true, false)).toBe(true);
    expect(logicalOrExercise(true, true)).toBe(true);
  });
});
