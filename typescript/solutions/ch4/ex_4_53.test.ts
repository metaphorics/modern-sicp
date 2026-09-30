// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { accumulatedPairs, pairRun } from "./ex_4_53.js";

describe("exercise 4.53: permanent accumulation under failure", () => {
  it("delivers the discovered prime-sum pairs in reverse discovery order", () => {
    expect(accumulatedPairs()).toStrictEqual(["[[8, 35], [3, 110], [3, 20]]"]);
  });

  it("records failures, retry steps, and completed exhaustion", () => {
    const run = pairRun();
    expect(run.failures).toBeGreaterThan(0);
    expect(run.steps).toBeGreaterThan(0);
    expect(run.status).toBe("completed");
  });
});
