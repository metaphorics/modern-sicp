// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { uniqueAnswers } from "./ex_4_75.js";

describe("exercise 4.75: the unique special form", () => {
  it("finds the one computer wizard", () => {
    const [wizard] = uniqueAnswers();
    expect(wizard).toStrictEqual(['unique(job(["Bitdiddle", "Ben"], ["computer", "wizard"]))']);
  });

  it("empties on the two programmers", () => {
    const [, programmers] = uniqueAnswers();
    expect(programmers).toStrictEqual(["No."]);
  });

  it("lists every singly filled job with its holder", () => {
    const [, , singletons] = uniqueAnswers();
    expect(singletons).toHaveLength(7);
    expect(singletons[0]).toBe(
      'and(job(["Bitdiddle", "Ben"], ["computer", "wizard"]), unique(job(["Bitdiddle", "Ben"], ["computer", "wizard"])))',
    );
    expect(singletons[6]).toContain('["Aull", "DeWitt"]');
  });

  it("finds everyone supervising precisely one person", () => {
    const [, , , supervisors] = uniqueAnswers();
    expect(supervisors).toStrictEqual([
      'and(supervisor(["Reasoner", "Louis"], ["Hacker", "Alyssa", "P"]), unique(supervisor(["Reasoner", "Louis"], ["Hacker", "Alyssa", "P"])))',
      'and(supervisor(["Cratchet", "Robert"], ["Scrooge", "Eben"]), unique(supervisor(["Cratchet", "Robert"], ["Scrooge", "Eben"])))',
    ]);
  });
});
