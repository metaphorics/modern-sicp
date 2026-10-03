// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { bigShotAnswers } from "./ex_4_58.js";

describe("exercise 4.58: the big-shot rule", () => {
  it("finds each division head with no supervisor in the division", () => {
    expect(bigShotAnswers()).toStrictEqual([
      'big-shot(["Bitdiddle", "Ben"], "computer")',
      'big-shot(["Warbucks", "Oliver"], "administration")',
      'big-shot(["Scrooge", "Eben"], "accounting")',
    ]);
  });
});
