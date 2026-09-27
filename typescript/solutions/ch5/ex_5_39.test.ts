// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_39 } from "./ex_5_39.js";

describe("exercise 5.39", () => {
  it("answers 120 through the lexical accesses", () => {
    const answers = ex_5_39();
    expect(answers[0]).toContain("120");
    expect(answers[1]).toContain("lexical-address-set!");
  });
});
