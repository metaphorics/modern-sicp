// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { applyRuleLocally, rewriteInfix } from "./ex_4_79.js";

describe("exercise 4.79: environments instead of renaming", () => {
  it("isolates parameters in a local binding environment", () =>
    expect(
      applyRuleLocally(
        ["x"],
        [4],
        (env) => {
          const { x, outer } = env;
          return [x, outer];
        },
        { x: 99, outer: 99 },
      ),
    ).toEqual([4, 99]));

  it("rewrites infix arithmetic", () => expect(rewriteInfix("(?x + 3)")).toBe("(+ ?x 3)"));
});
