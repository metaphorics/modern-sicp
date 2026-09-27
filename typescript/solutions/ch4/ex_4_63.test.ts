// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_63, genealogyAnswers } from "./ex_4_63.js";

describe("exercise 4.63: genealogy", () => {
  it("deduces Cain's grandson, Lamech's sons, and Methushael's grandsons", () => {
    const [cainsGrandsons, lamechsSons, methushaelsGrandsons] = genealogyAnswers();
    expect(cainsGrandsons).toStrictEqual(["(grandson Cain Irad)"]);
    expect(lamechsSons).toStrictEqual(["(son Lamech Jabal)", "(son Lamech Jubal)"]);
    expect(methushaelsGrandsons).toStrictEqual([
      "(grandson Methushael Jabal)",
      "(grandson Methushael Jubal)",
    ]);
  });

  it("summarizes the three deductions", () => {
    expect(ex_4_63()).toContain("Jabal");
  });
});
