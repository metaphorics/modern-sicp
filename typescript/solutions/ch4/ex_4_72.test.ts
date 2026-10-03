// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { interleaveVsAppend } from "./ex_4_72.js";

describe("exercise 4.72: interleave versus append", () => {
  it("surfaces rule answers early when interleaved, last when appended", () => {
    const [interleaved, appended] = interleaveVsAppend();
    expect(interleaved[1]).toBe('son("Lamech", "Jabal")');
    expect(appended[8]).toBe('son("Lamech", "Jabal")');
  });
});
