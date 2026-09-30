// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { countAnswers, countRun } from "./ex_4_51.js";

describe("exercise 4.51: reversible set! versus permanent assignment", () => {
  it("restores set! after failed branches but keeps permanent writes", () => {
    expect(countAnswers("reversible")).toStrictEqual([
      '["a", "b", 1]',
      '["a", "c", 1]',
      '["b", "a", 1]',
      '["b", "c", 1]',
      '["c", "a", 1]',
      '["c", "b", 1]',
    ]);
    expect(countAnswers("permanent")).toStrictEqual([
      '["a", "b", 2]',
      '["a", "c", 3]',
      '["b", "a", 4]',
      '["b", "c", 6]',
      '["c", "a", 7]',
      '["c", "b", 8]',
    ]);
  });

  it("reports real failure and continuation counts for both runs", () => {
    const reversible = countRun("reversible");
    const permanent = countRun("permanent");
    expect(reversible.failures).toBeGreaterThan(0);
    expect(reversible.steps).toBeGreaterThan(0);
    expect(permanent.failures).toBe(reversible.failures);
    expect(permanent.steps).toBe(reversible.steps);
    expect(reversible.status).toBe("completed");
    expect(permanent.status).toBe("completed");
  });
});
