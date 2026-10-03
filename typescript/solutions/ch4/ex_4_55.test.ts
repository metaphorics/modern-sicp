// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { simpleQueryAnswers } from "./ex_4_55.js";

describe("exercise 4.55: simple queries", () => {
  it("retrieves the requested people and addresses in database order", () => {
    const [supervisees, accountants, residents] = simpleQueryAnswers();
    expect(supervisees).toStrictEqual([
      'supervisor(["Hacker", "Alyssa", "P"], ["Bitdiddle", "Ben"])',
      'supervisor(["Fect", "Cy", "D"], ["Bitdiddle", "Ben"])',
      'supervisor(["Tweakit", "Lem", "E"], ["Bitdiddle", "Ben"])',
    ]);
    expect(accountants).toStrictEqual([
      'job(["Scrooge", "Eben"], ["accounting", "chief", "accountant"])',
      'job(["Cratchet", "Robert"], ["accounting", "scrivener"])',
    ]);
    expect(residents).toStrictEqual([
      'address(["Bitdiddle", "Ben"], ["Slumerville", ["Ridge", "Road"], "10"])',
      'address(["Reasoner", "Louis"], ["Slumerville", ["Pine", "Tree", "Road"], "80"])',
      'address(["Aull", "DeWitt"], ["Slumerville", ["Onion", "Square"], "5"])',
    ]);
  });
});
