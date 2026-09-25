// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeOrderProbe } from "./ex_3_08.js";

describe("exercise 3.8: operand evaluation order", () => {
  it("f(0) + f(1) answers 0: JavaScript evaluates operands left to right", () => {
    const f = makeOrderProbe();
    expect(f(0) + f(1)).toBe(0);
  });

  it("f(1) + f(0) answers 1: the first call executed decides the state", () => {
    const f = makeOrderProbe();
    expect(f(1) + f(0)).toBe(1);
  });

  it("a fresh probe starts from state 1 every time", () => {
    const f = makeOrderProbe();
    expect(f(5)).toBe(5);
    const g = makeOrderProbe();
    expect(g(5)).toBe(5);
  });
});
