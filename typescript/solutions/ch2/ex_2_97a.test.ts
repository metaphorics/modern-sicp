// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { poly97 } from "./ex_2_97.js";
import {
  gcdAfterReduce,
  isUnitTermList,
  reducedInstance,
  reduceRoundTrip,
  showTerms,
} from "./ex_2_97a.js";

// The book's 2.97 example pair: N = x^4+x^3+x^2-2x-1 over
// D = x^5-x^3-x^2+1, unreduced.
const nTerms = poly97([
  [4n, 1n],
  [3n, 1n],
  [2n, 1n],
  [1n, -2n],
  [0n, -1n],
]).contents.terms;
const dTerms = poly97([
  [5n, 1n],
  [3n, -1n],
  [2n, -1n],
  [0n, 1n],
]).contents.terms;

describe("exercise 2.97a: nothing left to drop after reduce", () => {
  it("gcd-terms of a reduced pair is a unit, degree zero", () => {
    const g = gcdAfterReduce(nTerms, dTerms);
    expect(g._tag).toBe("Ok");
    if (g._tag !== "Ok") {
      return;
    }
    expect(isUnitTermList(g.value)).toBe(true);
    expect(showTerms(g.value)).toBe("(polynomial x (0 1))");
  });

  it("reducing a reduced pair answers the same pair", () => {
    const r = reduceRoundTrip(nTerms, dTerms);
    expect(r._tag).toBe("Ok");
    if (r._tag !== "Ok") {
      return;
    }
    expect(showTerms(r.value.twice[0])).toBe(showTerms(r.value.once[0]));
    expect(showTerms(r.value.twice[1])).toBe(showTerms(r.value.once[1]));
    expect(showTerms(r.value.once[0])).toBe("(polynomial x (3 1) (2 2) (1 3) (0 1))");
    expect(showTerms(r.value.once[1])).toBe("(polynomial x (4 1) (3 1) (1 -1) (0 -1))");
  });

  it("reduces the 2.95 products to exactly P2 over P3", () => {
    const r = reducedInstance();
    expect(r._tag).toBe("Ok");
    if (r._tag !== "Ok") {
      return;
    }
    expect(showTerms(r.value.numer)).toBe("(polynomial x (2 11) (0 7))");
    expect(showTerms(r.value.denom)).toBe("(polynomial x (1 13) (0 5))");
  });
});
