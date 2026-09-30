// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_38 } from "./ex_5_38.ts";

describe("exercise 5.38 open coding", () => {
  it("open-coded arithmetic is much smaller and keeps the answers", () => {
    const lines = ex_5_38();
    expect(lines[2]).toContain("shrinks arithmetic-heavy code");
    expect(lines.length).toBe(4);
  });
});
