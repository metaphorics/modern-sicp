// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { replacementAnswers } from "./ex_4_57.js";

describe("exercise 4.57: the can-replace rule", () => {
  it("replaces Fect through the shared job and the wizard step", () => {
    const [fect] = replacementAnswers();
    expect(fect).toStrictEqual([
      'can-replace(["Hacker", "Alyssa", "P"], ["Fect", "Cy", "D"])',
      'can-replace(["Bitdiddle", "Ben"], ["Fect", "Cy", "D"])',
    ]);
  });

  it("keeps only replacements favoring the better-paid side", () => {
    const [, more] = replacementAnswers();
    expect(more).toHaveLength(2);
    expect(more).toEqual(
      expect.arrayContaining([
        'and(can-replace(["Fect", "Cy", "D"], ["Hacker", "Alyssa", "P"]), salary(["Fect", "Cy", "D"], "35000"), salary(["Hacker", "Alyssa", "P"], "40000"), lisp-value("35000", "40000"))',
        'and(can-replace(["Aull", "DeWitt"], ["Warbucks", "Oliver"]), salary(["Aull", "DeWitt"], "25000"), salary(["Warbucks", "Oliver"], "150000"), lisp-value("25000", "150000"))',
      ]),
    );
  });
});
