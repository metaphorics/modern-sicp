// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_40 } from "./ex_5_40.ts";

describe("exercise 5.40 compile-time environment threading", () => {
  it("reports the frame each reference is compiled against", () => {
    const lines = ex_5_40();
    expect(lines[0]).toContain("[[y, z], [a, b, c, d, e], [x, y]]");
    expect(lines.some((line) => line.startsWith("x: frame 2"))).toBe(true);
    expect(lines.some((line) => line.startsWith("a: frame 1"))).toBe(true);
  });
});
