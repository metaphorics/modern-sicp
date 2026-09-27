// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_43 } from "./ex_5_43.js";

describe("exercise 5.43", () => {
  it("scans out the internal definitions", () => {
    const answers = ex_5_43();
    expect(answers[0]).toContain("define-variable!");
    expect(answers[1]).toContain("*unassigned*");
    expect(answers[2]).toContain("3");
  });
});
