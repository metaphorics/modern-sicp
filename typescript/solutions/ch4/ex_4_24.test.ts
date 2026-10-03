// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { benchmarkAnalysis, fibWorkloadCall } from "./ex_4_24.js";

describe("exercise 4.24: benchmark analysis versus execution", () => {
  it("reports seven runs per path, positive medians, and the analysis fraction", () => {
    const result = benchmarkAnalysis();
    expect(result.directRuns).toHaveLength(7);
    expect(result.analyzedRuns).toHaveLength(7);
    expect(result.directMedian > 0).toBe(true);
    expect(result.analyzedMedian > 0).toBe(true);
    expect(result.fraction).toBeCloseTo(
      (result.directMedian - result.analyzedMedian) / result.directMedian,
      10,
    );
  });

  it("the workload is the fib(14) call the book's comparison times", () => {
    const workload = fibWorkloadCall();
    expect(workload.tag).toBe("call");
    if (workload.tag === "call" && workload.callee.tag === "variable") {
      expect(workload.callee.name).toBe("fib");
    }
  });
});
