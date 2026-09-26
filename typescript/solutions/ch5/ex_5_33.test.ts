// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_33 } from "./ex_5_33.js";

describe("exercise 5.33", () => {
  it("runs both orders and reports their sizes", () => {
    const answers = ex_5_33();
    expect(answers[0]).toMatch(/\d+ statements, \d+ save sites/);
    expect(answers[1]).toMatch(/\d+ statements, \d+ save sites/);
    expect(answers[2]).toContain("120");
    expect(answers[3]).toContain("120");
    expect(answers[4]).toContain("which register is live");
  });
});
