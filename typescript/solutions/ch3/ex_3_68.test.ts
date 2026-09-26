// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { integers, streamRef, streamTake } from "../../packages/ch3/src/05-streams.js";

import {
  LouisBudgetExceeded,
  louisPairs,
  louisPairsPlainArgument,
  makeLouisMeter,
} from "./ex_3_68.js";

describe("exercise 3.68: Louis's whole-first-row pairs", () => {
  it("serves the first pair (1, 1) with one metered step", () => {
    const meter = makeLouisMeter(200);
    const louis = louisPairs(meter, integers, integers);
    expect(streamRef(louis, 0)).toEqual([1, 1]);
    expect(meter.steps()).toBe(1);
  });

  it("the whole visible stream is the first row: elements 1 to 199 are (1, j)", () => {
    const meter = makeLouisMeter(200);
    const louis = louisPairs(meter, integers, integers);
    expect(streamTake(louis, 199)).toEqual(Array.from({ length: 199 }, (_, idx) => [1, idx + 1]));
    expect(meter.steps()).toBe(200);
  });

  it("demanding past the first row burns the budget and aborts at step 201", () => {
    const meter = makeLouisMeter(200);
    const louis = louisPairs(meter, integers, integers);
    expect(() => streamTake(louis, 200)).toThrow(LouisBudgetExceeded);
    expect(meter.steps()).toBe(201);
  });

  it("the statement's plain-argument rest diverges before any element exists", () => {
    const meter = makeLouisMeter(200);
    expect(() => louisPairsPlainArgument(meter, integers, integers)).toThrow(LouisBudgetExceeded);
    expect(meter.steps()).toBe(201);
  });
});
