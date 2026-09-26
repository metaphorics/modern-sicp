// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  streamEnumerateInterval,
  streamRef,
  streamTake,
} from "../../packages/ch3/src/05-streams.js";

import { delayedIntegral, solve, solveWithPlainIntegral } from "./ex_3_77.js";

describe("exercise 3.77: integral with a delayed integrand", () => {
  it("emits the initial value without forcing the integrand", () => {
    const bomb = delayedIntegral(
      () => {
        throw new Error("integrand forced");
      },
      7,
      0.5,
    );
    expect(streamRef(bomb, 0)).toBe(7);
    expect(() => streamRef(bomb, 1)).toThrow("integrand forced");
  });

  it("solves dy/dt = y from y0 = 1 at dt = 0.001 toward e", () => {
    const y = solve((v) => v, 1, 0.001);
    expect(streamRef(y, 1000)).toBe(2.716923932235896);
    expect(streamRef(y, 1000)).toBeCloseTo(2.716924, 5);
  });

  it("ends when the delayed integrand runs out", () => {
    const ys = delayedIntegral(() => streamEnumerateInterval(1, 3), 10, 2);
    expect(streamTake(ys, 6)).toEqual([10, 12, 16, 22]);
  });

  it("cannot even be constructed with the plain undelayed integral", () => {
    expect(() => solveWithPlainIntegral((v) => v, 1, 0.001)).toThrow(ReferenceError);
    expect(() => solveWithPlainIntegral((v) => v, 1, 0.001)).toThrow(
      "Cannot access 'y' before initialization",
    );
  });
});
