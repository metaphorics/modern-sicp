// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_18 } from "./ex_5_18.ts";

describe("exercise 5.18 traced registers", () => {
  it("reports every store of the controller's registers", () => {
    const result = ex_5_18();
    expect(result.answer).toBe(4);
    const tail = result.log.slice(-2);
    expect(tail[0]).toBe("a: 8 -> 4");
    expect(tail[1]).toBe("b: 4 -> 0");
    expect(result.log.length).toBeGreaterThan(0);
  });
});
