// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_52 } from "./ex_5_52.ts";

describe("exercise 5.52", { timeout: 300_000 }, () => {
  it("emits C from the typed stream and the built interpreter matches the direct evaluator", () => {
    const lines = ex_5_52();
    expect(lines.length).toBeGreaterThan(0);
    expect(lines.some((line) => line.startsWith("Error:"))).toBe(false);
  });
});
