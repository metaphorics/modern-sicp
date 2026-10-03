// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_47 } from "./ex_5_47.ts";

describe("exercise 5.47 compiled code calls interpreted procedures", () => {
  it("the two engines agree on the mixed session", () => {
    const lines = ex_5_47();
    expect(lines[lines.length - 1]).toContain("agree on the mixed calls");
  });
});
