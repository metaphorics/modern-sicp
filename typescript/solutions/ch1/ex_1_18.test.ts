// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { timesIter, timesIterStates } from "./ex_1_18.js";

describe("exercise 1.18", () => {
  it("lands on the printed products", () => {
    expect(timesIter(17, 31)).toBe(527);
    expect(timesIter(2, 10)).toBe(20);
    expect(timesIter(7, 0)).toBe(0);
  });

  it("agrees with the host product over a grid", () => {
    for (let a = 1; a <= 12; a += 1) {
      for (let b = 0; b <= 12; b += 1) {
        expect(timesIter(a, b)).toBe(a * b);
      }
    }
  });

  it("x * y + acc is the product at every state", () => {
    const states = timesIterStates(17, 31);
    expect(states.length).toBe(10);
    for (const s of states) {
      expect(s.x * s.y + s.acc).toBe(527);
    }
    expect(timesIterStates(12345, 6789).length).toBe(19);
    expect(timesIter(12345, 6789)).toBe(83810205);
  });
});
