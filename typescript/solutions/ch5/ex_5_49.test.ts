// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_49 } from "./ex_5_49.js";

describe("exercise 5.49", () => {
  it("runs the chained compiled forms", () => {
    const answers = ex_5_49();
    expect(answers[0]).toContain("144");
    expect(answers[0]).toContain("882");
  });
});
