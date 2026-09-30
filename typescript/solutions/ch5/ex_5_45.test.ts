// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_45, ex_5_46 } from "./ex_5_45.ts";

describe("exercises 5.45 and 5.46 stack ratios", () => {
  it("reports the interpreted counters beside the compiled save traffic", () => {
    for (const line of [...ex_5_45(), ...ex_5_46()]) {
      expect(line).toContain("interpreted pushes");
      expect(line).toContain("compiled saves");
    }
  });
});
