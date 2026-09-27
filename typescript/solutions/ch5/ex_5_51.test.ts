// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_51 } from "./ex_5_51.js";

describe("exercise 5.51", { timeout: 120_000 }, () => {
  it("builds and runs the C evaluator on the factorial session", () => {
    const output = ex_5_51();
    expect(output[0]).toContain("ok");
    expect(output[0]).toContain("120");
  });
});
