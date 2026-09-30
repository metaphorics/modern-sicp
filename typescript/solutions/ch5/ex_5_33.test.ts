// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_33, summary } from "./ex_5_33.ts";

describe("exercise 5.33 factorial-alt compilation", () => {
  it("the two orderings compile to the same shape", () => {
    const alt = summary(
      "function factorialAlt(n: number): number { return n === 1 ? 1 : n * factorialAlt(n - 1); }",
    );
    const book = summary(
      "function factorial(n: number): number { return n === 1 ? 1 : factorial(n - 1) * n; }",
    );
    expect(alt.statements).toBe(book.statements);
    expect(alt.saves).toBe(book.saves);
  });
  it("reports the comparison", () => {
    expect(ex_5_33()[2]).toContain("operand order");
  });
});
