// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";
import { operandOrderSource } from "./ex_4_46.js";

describe("exercise 4.46: left-to-right operands", () => {
  it("the answers enumerate left-major and the stream logs once per choice-point visit", () => {
    const run = runAmbAnswers(operandOrderSource, "amb-depth-first-experiment", 1);
    expect(run.answers.map((value) => format(value))).toEqual([
      "[1, 1]",
      "[1, 2]",
      "[2, 1]",
      "[2, 2]",
    ]);
    expect(run.transcript).toEqual(["left", "right", "right"]);
  });
});
