// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamRef, streamTake } from "../../packages/ch3/src/05-streams.js";

import { solveGeneral } from "./ex_3_79.js";

describe("exercise 3.79: the general second-order solver", () => {
  it("solves y'' = y' with f receiving the derivative first", () => {
    const y = solveGeneral((d) => d, 1, 2, 0.25);
    expect(streamTake(y, 5)).toEqual([1, 1.5, 2.125, 2.90625, 3.8828125]);
  });

  it("reproduces the undamped oscillator through an f closure", () => {
    const y = solveGeneral((_d, yy) => -yy, 0, 1, 0.01);
    const y1 = streamRef(y, 100);
    const y2 = streamRef(y, 200);
    const y3 = streamRef(y, 300);
    expect(y1).toBe(0.8456705645316805);
    expect(y2).toBe(0.918463576916705);
    expect(y3).toBe(0.14335314503859142);
    expect(Math.abs(y1 - Math.sin(1))).toBe(0.0041995797237840415);
    expect(Math.abs(y2 - Math.sin(2))).toBe(0.009166150091023284);
    expect(Math.abs(y3 - Math.sin(3))).toBe(0.0022331369787242095);
  });

  it("reproduces the damped oscillator through an f closure", () => {
    const y = solveGeneral((d, yy) => -0.1 * d - yy, 1, 0, 0.01);
    expect(streamTake(y, 5)).toEqual([1, 1, 0.9999, 0.9997001, 0.9994004099]);
    expect(streamRef(y, 500)).toBe(0.18510721623125634);
    expect(streamRef(y, 4000)).toBe(-0.09990880783812048);
    expect(Math.abs(streamRef(y, 4000))).toBeLessThan(Math.abs(streamRef(y, 500)));
  });
});
