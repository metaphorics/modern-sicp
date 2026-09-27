// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeAgenda, makeWire, propagate } from "../../packages/ch3/src/03-mutable-data.js";

import { answer, bitsToNumber, numberToBits, rippleCarryAdder } from "./ex_3_30.js";

/** Builds one adder over k bits, sets the operand wires, propagates,
 * and returns the computed answer. */
const runAddition = (a: number, b: number, k: number): number => {
  const agenda = makeAgenda();
  const listA = numberToBits(a, k);
  const listB = numberToBits(b, k);
  const listS = Array.from({ length: k }, () => makeWire());
  const cIn = makeWire();
  const carryOut = rippleCarryAdder(listA, listB, listS, cIn, agenda);
  propagate(agenda);
  return answer(listS, carryOut);
};

describe("exercise 3.30: the ripple-carry adder", () => {
  it("adds two k-bit numbers: the book's carry ripples stage to stage", () => {
    expect(runAddition(13, 7, 4)).toBe(20);
    expect(runAddition(5, 5, 4)).toBe(10);
    expect(runAddition(15, 1, 4)).toBe(16);
  });

  it("checks a spread of operand pairs against integer addition", () => {
    for (const [a, b] of [
      [0, 0],
      [1, 0],
      [0, 1],
      [7, 8],
      [6, 9],
      [12, 3],
      [10, 10],
    ] as const) {
      expect(runAddition(a, b, 4)).toBe(a + b);
    }
  });

  it("bit helpers round-trip, and a length mismatch fails", () => {
    const bits = numberToBits(13, 4);
    expect(bitsToNumber(bits)).toBe(13);
    expect(bits[0]?.getSignal()).toBe(true);
    expect(bits[2]?.getSignal()).toBe(true);
    const agenda = makeAgenda();
    expect(() =>
      rippleCarryAdder(
        numberToBits(1, 3),
        numberToBits(1, 4),
        Array.from({ length: 4 }, () => makeWire()),
        makeWire(),
        agenda,
      ),
    ).toThrow("ripple-carry adder: operand and sum lists must have equal length");
  });
});
