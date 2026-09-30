// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { familyAnswers } from "./ex_4_63.js";

describe("exercise 4.63: family rules", () => {
  it("finds Cain's grandson through the son chain", () => {
    const [cain] = familyAnswers();
    expect(cain).toStrictEqual(['grandson("Irad", "Cain")']);
  });

  it("finds Lamech's sons through the wife rule", () => {
    const [, lamech] = familyAnswers();
    expect(lamech).toStrictEqual(['son("Lamech", "Jabal")', 'son("Lamech", "Jubal")']);
  });

  it("finds Methushael's grandsons through both rules", () => {
    const [, , methushael] = familyAnswers();
    expect(methushael).toStrictEqual([
      'grandson("Jabal", "Methushael")',
      'grandson("Jubal", "Methushael")',
    ]);
  });
});
