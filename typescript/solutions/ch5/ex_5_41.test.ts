// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_41, findVariable } from "./ex_5_41.ts";

describe("exercise 5.41 find-variable", () => {
  it("answers a lexical address for bound names and free otherwise", () => {
    const env = [["x"], ["y", "z"]];
    expect(findVariable("x", env)).toEqual({ found: true, frame: 0, position: 0 });
    expect(findVariable("z", env)).toEqual({ found: true, frame: 1, position: 1 });
    expect(findVariable("free", env)).toEqual({ found: false });
  });
  it("walks the book's example environment", () => {
    const lines = ex_5_41();
    expect(lines[0]).toContain("frame 0");
    expect(lines[4]).toContain("free");
  });
});
