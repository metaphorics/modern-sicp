// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_44, isOpenCoded } from "./ex_5_44.ts";

describe("exercise 5.44 open coding respects shadowing", () => {
  it("a bound name is never open-coded", () => {
    expect(isOpenCoded("+", [["+"]])).toBe(false);
    expect(isOpenCoded("+", [["x"]])).toBe(true);
    expect(isOpenCoded("f", [["x"]])).toBe(false);
  });
  it("reports the three probes", () => {
    expect(ex_5_44()[1]).toContain("false");
    expect(ex_5_44()[0]).toContain("true");
  });
});
