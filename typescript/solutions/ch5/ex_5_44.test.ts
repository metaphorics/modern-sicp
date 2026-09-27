// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_44 } from "./ex_5_44.js";

describe("exercise 5.44", () => {
  it("respects the shadowing of open-coded names", () => {
    const answers = ex_5_44();
    expect(answers[0]).toContain(": 0 ");
    expect(answers[1]).not.toContain(": 0 ");
  });
});
