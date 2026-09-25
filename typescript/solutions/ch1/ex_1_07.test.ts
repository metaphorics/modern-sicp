// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { sqrtRelative } from "./ex_1_07.js";

/** The section's absolute-tolerance sqrt, with a step cap for the probes. */
function oldSqrtCapped(x: number, maxSteps: number): { guess: number; converged: boolean } {
  const goodEnough = (guess: number): boolean => Math.abs(guess * guess - x) < 0.001;
  let guess = 1.0;
  for (let steps = 0; steps < maxSteps; steps += 1) {
    if (goodEnough(guess)) {
      return { guess, converged: true };
    }
    guess = (guess + x / guess) / 2;
  }
  return { guess, converged: goodEnough(guess) };
}

describe("exercise 1.7", () => {
  it("the absolute test fails small numbers: sqrt(0.0001) lands near 0.0323", () => {
    const { guess } = oldSqrtCapped(0.0001, 500);
    expect(guess).toBe(0.03230844833048122);
    expect(guess).toBeCloseTo(0.0323, 3);
  });

  it("the absolute test fails large numbers: sqrt(1234567890123456800000) never settles", () => {
    const { converged } = oldSqrtCapped(1234567890123456800000, 300);
    expect(converged).toBe(false);
  });

  it("the relative test gets small numbers right", () => {
    expect(sqrtRelative(0.0001)).toBeCloseTo(0.01, 6);
    expect(sqrtRelative(9)).toBeCloseTo(3, 6);
  });

  it("the relative test gets large numbers right", () => {
    const large = sqrtRelative(1234567890123456800000);
    const oracle = Math.sqrt(1234567890123456800000);
    expect(Math.abs(large - oracle) < 0.001 * oracle).toBe(true);
  });
});
