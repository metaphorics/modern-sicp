// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_40 } from "./ex_5_40.js";

describe("exercise 5.40", () => {
  it("threads the compile-time environment and reports frames", () => {
    const trace = ex_5_40();
    expect(trace).toContain("x in ((y z) (a b c d e) (x y))");
    expect(trace).toContain("z in ((y z) (a b c d e) (x y))");
  });
});
