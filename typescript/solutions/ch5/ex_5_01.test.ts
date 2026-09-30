// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_01 } from "./ex_5_01.ts";

describe("exercise 5.1 iterative factorial machine", () => {
  it("computes factorial through repeated controller iterations", () => {
    expect(ex_5_01(5)).toBe(120);
    expect(ex_5_01(1)).toBe(1);
  });
});
