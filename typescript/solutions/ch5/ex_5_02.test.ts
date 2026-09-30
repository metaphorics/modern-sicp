// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_02 } from "./ex_5_02.ts";

describe("exercise 5.2 iterative factorial controller", () => {
  it("answers one for zero and factorial for positive input", () => {
    expect(ex_5_02(0)).toBe(1);
    expect(ex_5_02(6)).toBe(720);
  });
});
