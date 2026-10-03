// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_08 } from "./ex_5_08.ts";

describe("exercise 5.8 duplicate labels", () => {
  it("a second definition of one label is a duplicate-label assembly fault", () => {
    const result = ex_5_08();
    expect(result.duplicate).not.toBeNull();
    expect(result.duplicate?.tag).toBe("duplicate-label");
  });
  it("a forward reference resolves to the instruction after the label", () => {
    expect(ex_5_08().forwardAnswer).toBe(2);
  });
});
