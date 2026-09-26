// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_47 } from "./ex_5_47.js";

describe("exercise 5.47", () => {
  it("calls the interpreted procedure and answers 12", () => {
    const answers = ex_5_47();
    expect(answers[0]).toContain("compound-apply");
    expect(answers[1]).toContain("12");
  });
});
