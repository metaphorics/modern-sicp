// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_17 } from "./ex_5_17.ts";

describe("exercise 5.17 labels in the trace", () => {
  it("every traced instruction carries the label in effect", () => {
    const result = ex_5_17();
    expect(result.lines).toHaveLength(26);
    for (const line of result.lines) {
      expect(line.startsWith("test-b: ")).toBe(true);
    }
    expect(result.answer).toBe(2);
  });
});
