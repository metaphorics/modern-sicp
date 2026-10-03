// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_09 } from "./ex_5_09.ts";

describe("exercise 5.9 labels are not operation operands", () => {
  it("rejects the book's offending controller with a bad-target fault", () => {
    const result = ex_5_09();
    expect(result.offending).not.toBeNull();
    expect(result.offending?.tag).toBe("bad-target");
  });
  it("accepts the controller once the operand is a value", () => {
    expect(ex_5_09().fixed).toBeNull();
  });
});
