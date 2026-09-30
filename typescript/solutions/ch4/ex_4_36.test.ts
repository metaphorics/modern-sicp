// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { format } from "../../packages/ch4/src/read.js";
import { answers } from "./ex_4_36.js";

describe("exercise 4.36: unbounded Pythagorean triples", () => {
  it("returns a bounded prefix from the unbounded hypotenuse-keyed search", () => {
    const run = answers();
    expect(run.answers.map((value) => format(value))).toEqual([
      "[3, 4, 5]",
      "[6, 8, 10]",
      "[5, 12, 13]",
      "[9, 12, 15]",
      "[8, 15, 17]",
      "[12, 16, 20]",
    ]);
    expect(run.answers).toHaveLength(6);
    expect(run.status).toBe("cut-off");
  });
});
