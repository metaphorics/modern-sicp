// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_55, simpleQueryAnswers } from "./ex_4_55.js";

describe("exercise 4.55: simple queries", () => {
  it("retrieves the requested people and addresses in database order", () => {
    const [supervisees, accountants, residents] = simpleQueryAnswers();
    expect(supervisees).toHaveLength(3);
    expect(supervisees[0]).toContain("(Hacker Alyssa P)");
    expect(supervisees[2]).toContain("(Tweakit Lem E)");
    expect(accountants).toHaveLength(2);
    expect(accountants[0]).toContain("(Scrooge Eben)");
    expect(accountants[1]).toContain("(Cratchet Robert)");
    expect(residents).toHaveLength(3);
    expect(residents[0]).toContain("(Bitdiddle Ben)");
    expect(residents[2]).toContain("(Aull DeWitt)");
  });

  it("summarizes all three executable lookups", () => {
    expect(ex_4_55()).toContain("3 people supervised by Ben");
  });
});
