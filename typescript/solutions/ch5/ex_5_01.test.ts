// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_01 } from "./ex_5_01.js";

describe("exercise 5.1 iterative factorial machine", () => {
  it("computes factorial through repeated controller iterations", () => {
    const result = ex_5_01(5);
    expect(result.ok).toBe(true);
    if (result.ok) expect(result.value.registers["product"]).toBe(120);
  });
});
