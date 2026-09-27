// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_69, familyAnswers } from "./ex_4_69.js";

describe("exercise 4.69: great-grandson rules", () => {
  it("finds Adam's great-grandson through the grandson anchor", () => {
    expect(familyAnswers("((great grandson) Adam ?who)")).toStrictEqual([
      "((great grandson) Adam Irad)",
    ]);
  });

  it("reaches Jabal and Jubal at five greats", () => {
    expect(familyAnswers("((great great great great great grandson) Adam ?who)")).toStrictEqual([
      "((great great great great great grandson) Adam Jabal)",
      "((great great great great great grandson) Adam Jubal)",
    ]);
    expect(ex_4_69()).toContain("Irad");
  });
});
