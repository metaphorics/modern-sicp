// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_16 } from "./ex_5_16.ts";

describe("exercise 5.16 instruction tracing", () => {
  it("the switched-on trace is the executed instructions in order", () => {
    const result = ex_5_16();
    expect(result.traced).toHaveLength(26);
    expect(result.traced[0]).toContain("test");
    expect(result.traced[25]).toContain("branch");
    expect(result.answer).toBe(2);
  });
  it("the switch off leaves the transcript empty", () => {
    expect(ex_5_16().silent).toEqual([]);
  });
});
