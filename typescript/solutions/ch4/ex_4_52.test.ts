// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ifFailAnswers, ifFailRun } from "./ex_4_52.js";

describe("exercise 4.52: if-fail", () => {
  it("delivers the fallback once after an all-odd primary search fails", () => {
    expect(ifFailAnswers("1, 3, 5")).toStrictEqual(['"all-odd"']);
  });

  it("answers 8 before delivering the fallback after primary exhaustion", () => {
    expect(ifFailAnswers("1, 3, 5, 8")).toStrictEqual(["8", '"all-odd"']);
  });

  it("exposes real failed-computation and deferred-step counts", () => {
    const odd = ifFailRun("1, 3, 5");
    const withEight = ifFailRun("1, 3, 5, 8");
    expect(odd.failures).toBeGreaterThan(0);
    expect(odd.steps).toBeGreaterThan(0);
    expect(withEight.failures).toBeGreaterThan(0);
    expect(withEight.steps).toBeGreaterThan(0);
    expect(odd.status).toBe("completed");
    expect(withEight.status).toBe("completed");
  });
});
