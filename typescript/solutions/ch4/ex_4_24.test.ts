// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  analysisFraction,
  benchmarkAnalyzed,
  benchmarkDirect,
  ex_4_24,
  medianOf,
} from "./ex_4_24.js";

describe("exercise 4.24: benchmark analysis versus execution", () => {
  const direct = benchmarkDirect();
  const analyzed = benchmarkAnalyzed();

  it("both evaluators produce positive medians over the same workload", () => {
    expect(direct.runs).toHaveLength(7);
    expect(analyzed.runs).toHaveLength(7);
    expect(direct.medianMs).toBeGreaterThan(0);
    expect(analyzed.medianMs).toBeGreaterThan(0);
  });

  it("the analyzed median is at most the direct median with noise margin", () => {
    expect(analyzed.medianMs).toBeLessThanOrEqual(direct.medianMs * 1.5);
  });

  it("the derived analysis fraction is a plausible share", () => {
    const fraction = analysisFraction(direct.medianMs, analyzed.medianMs);
    expect(fraction).toBeGreaterThan(-0.5);
    expect(fraction).toBeLessThanOrEqual(1);
  });

  it("medianOf answers the middle of a sorted copy", () => {
    expect(medianOf([5, 1, 3])).toBe(3);
    expect(medianOf([4, 1, 3, 2])).toBe(2.5);
  });

  it("reports medians and the analysis fraction", () => {
    const report = ex_4_24();
    expect(report).toMatch(/direct \d/);
    expect(report).toMatch(/analyzed \d/);
  });
});
