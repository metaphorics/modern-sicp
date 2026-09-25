// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { fixedPointReport, solveXtoTheX } from "./ex_1_36.js";

describe("exercise 1.36", () => {
  it("reports every approximation on the way to the cosine fixed point", () => {
    const guesses: number[] = [];
    const result = fixedPointReport(Math.cos, 1.0, (guess) => {
      guesses.push(guess);
    });
    expect(result).toBe(0.7390822985224023);
    expect(guesses.length).toBeGreaterThan(10);
    expect(guesses[guesses.length - 1]).toBe(result);
    expect(guesses[0]).toBeCloseTo(Math.cos(1), 15);
  });

  it("solves x^x = 1000, damping doing the work", () => {
    const { value, undampedSteps, dampedSteps } = solveXtoTheX();
    expect(value).toBe(4.555537551999825);
    expect(undampedSteps).toBe(34);
    expect(dampedSteps).toBe(9);
  });

  it("the answer satisfies x^x = 1000 within the tolerance", () => {
    const { value } = solveXtoTheX();
    expect(value * Math.log(value)).toBeCloseTo(Math.log(1000), 4);
  });
});
