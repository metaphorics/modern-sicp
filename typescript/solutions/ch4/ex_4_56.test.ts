// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { compoundAnswers, ex_4_56 } from "./ex_4_56.js";

describe("exercise 4.56: compound queries", () => {
  it("joins supervisor, address, salary, and division facts", () => {
    const [addresses, lowerPaid, outsideComputer] = compoundAnswers();
    expect(addresses).toHaveLength(3);
    expect(addresses[0]).toContain("(Hacker Alyssa P)");
    expect(lowerPaid.some((answer) => answer.includes("(Cratchet Robert)"))).toBe(true);
    expect(lowerPaid.every((answer) => answer.includes("30000"))).toBe(false);
    expect(outsideComputer).toHaveLength(4);
    expect(outsideComputer.some((answer) => answer.includes("(Aull DeWitt)"))).toBe(true);
    expect(outsideComputer.some((answer) => answer.includes("(Cratchet Robert)"))).toBe(true);
    expect(outsideComputer.every((answer) => answer.includes("(Reasoner Louis)"))).toBe(false);
  });

  it("reports results from all requested compounds", () => {
    expect(ex_4_56()).toContain("supervisee addresses");
  });
});
