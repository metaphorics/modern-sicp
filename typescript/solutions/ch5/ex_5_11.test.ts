// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_11 } from "./ex_5_11.ts";

describe("exercise 5.11 the three save and restore disciplines", () => {
  it("the disciplines disagree exactly on the out-of-order sequence", () => {
    const result = ex_5_11();
    expect(result.blind).toBe(2);
    expect(result.checking?.tag).toBe("restore-mismatch");
    expect(result.perRegister).toBe(1);
  });
  it("all three answer the Fibonacci machine", () => {
    const result = ex_5_11().fib;
    expect(result.blind).toBe(2);
    expect(result.checking).toBe(2);
    expect(result.perRegister).toBe(2);
  });
  it("the part-(a) elimination works name-blind and is refused checked", () => {
    const collapsed = ex_5_11().collapsed;
    expect(collapsed.blind[0]).toBe(2);
    expect(collapsed.blind[1]).toBe(5);
    expect(collapsed.checking?.tag).toBe("restore-mismatch");
  });
});
