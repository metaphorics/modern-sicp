// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { compoundAnswers } from "./ex_4_56.js";

describe("exercise 4.56: compound queries", () => {
  it("conjoins supervisees with addresses in database order", () => {
    const [addresses] = compoundAnswers();
    expect(addresses).toStrictEqual([
      'and(supervisor(["Hacker", "Alyssa", "P"], ["Bitdiddle", "Ben"]), address(["Hacker", "Alyssa", "P"], ["Cambridge", ["Mass", "Ave"], "78"]))',
      'and(supervisor(["Fect", "Cy", "D"], ["Bitdiddle", "Ben"]), address(["Fect", "Cy", "D"], ["Cambridge", ["Ames", "Street"], "3"]))',
      'and(supervisor(["Tweakit", "Lem", "E"], ["Bitdiddle", "Ben"]), address(["Tweakit", "Lem", "E"], ["Boston", ["Bay", "State", "Road"], "22"]))',
    ]);
  });

  it("filters salaries below Ben's with both amounts shown", () => {
    const [, salaries] = compoundAnswers();
    expect(salaries).toHaveLength(6);
    expect(salaries[0]).toBe(
      'and(salary(["Hacker", "Alyssa", "P"], "40000"), salary(["Bitdiddle", "Ben"], "60000"), lisp-value("40000", "60000"))',
    );
    expect(salaries[5]).toBe(
      'and(salary(["Aull", "DeWitt"], "25000"), salary(["Bitdiddle", "Ben"], "60000"), lisp-value("25000", "60000"))',
    );
  });

  it("negates computer-division supervisors without binding the pattern tail", () => {
    const [, , outside] = compoundAnswers();
    expect(outside).toHaveLength(4);
    expect(outside[0]).toBe(
      'and(supervisor(["Bitdiddle", "Ben"], ["Warbucks", "Oliver"]), not(job(["Warbucks", "Oliver"], ["computer" | ?division-rest])), job(["Warbucks", "Oliver"], ["administration", "big", "wheel"]))',
    );
    expect(outside[2]).toContain('["Cratchet", "Robert"]');
  });

  it("addition 4.56a: keeps only the non-programmer supervisee", () => {
    const [, , , nonProgrammers] = compoundAnswers();
    expect(nonProgrammers).toStrictEqual([
      'and(supervisor(["Tweakit", "Lem", "E"], ["Bitdiddle", "Ben"]), not(job(["Tweakit", "Lem", "E"], ["computer", "programmer"])))',
    ]);
  });
});
