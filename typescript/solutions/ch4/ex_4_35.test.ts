// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { format } from "../../packages/ch4/src/read.js";
import { answers } from "./ex_4_35.js";

describe("exercise 4.35: an-integer-between and triples", () => {
  it("the triples between 1 and 20 come out in ascending i, j, k order", () => {
    const run = answers();
    expect(run.answers.map((value) => format(value))).toEqual([
      "[3, 4, 5]",
      "[5, 12, 13]",
      "[6, 8, 10]",
      "[8, 15, 17]",
      "[9, 12, 15]",
      "[12, 16, 20]",
    ]);
  });
});
