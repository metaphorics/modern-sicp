// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_52 } from "./ex_5_52.js";

describe("exercise 5.52", { timeout: 300_000 }, () => {
  it("emits C, builds it, and runs the object session", () => {
    const output = ex_5_52();
    expect(output[0]).toContain("120");
    expect(output[0]).toContain("(tick tick tick)");
  });
});
