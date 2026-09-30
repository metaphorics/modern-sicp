// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { makeEvaluator } from "../../packages/ch5/src/04-eceval.ts";
import { ITERATIVE_FACTORIAL } from "./ex_5_26.ts";
import { deTailedController, ex_5_28 } from "./ex_5_28.ts";

describe("exercise 5.28 remove the tail shortcut and rerun", () => {
  it("the de-tailed machine never uses less stack than the base machine", () => {
    const lines = ex_5_28();
    for (const line of lines.slice(0, 2)) {
      const parts = line.match(/base depth step = (\d+), de-tailed = (\d+)/);
      expect(parts).not.toBeNull();
      const base = Number(parts?.[1] ?? 0);
      const plain = Number(parts?.[2] ?? 0);
      expect(plain).toBeGreaterThanOrEqual(base);
    }
  });
  it("the answers are unchanged on the variant", () => {
    const result = makeEvaluator(
      `${ITERATIVE_FACTORIAL}\nconsole.log(factorial(5));`,
      {},
      deTailedController,
    ).run();
    expect(result.outcome.tag).toBe("ok");
    expect(result.transcript.some((line) => line.includes("120"))).toBe(true);
  });
});
