// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_37 } from "./ex_5_37.ts";

describe("exercise 5.37 preserving disabled", () => {
  it("compares the shipped discipline with the indiscriminate one", () => {
    const lines = ex_5_37();
    expect(lines[0]).toContain("shipped:");
    expect(lines[1]).toContain("indiscriminate wrapper");
    expect(lines[2]).toContain("answers are unchanged");
  });
});
