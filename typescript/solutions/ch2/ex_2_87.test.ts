// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  adjoinTerm,
  isZeroQ,
  makePolynomial,
  makeSchemeNumber,
  show,
} from "../../packages/ch2/src/05-generic-operations.js";

const sn = (n: bigint) => makeSchemeNumber(n);

describe("exercise 2.87: polynomial =zero?", () => {
  it("answers true for the empty polynomial", () => {
    expect(show(isZeroQ(makePolynomial("x", [])))).toBe("true");
  });

  it("answers true when every coefficient is zero", () => {
    expect(
      show(
        isZeroQ(
          makePolynomial("x", [
            [2n, sn(0n)],
            [0n, sn(0n)],
          ]),
        ),
      ),
    ).toBe("true");
  });

  it("answers false as soon as one coefficient is nonzero", () => {
    expect(
      show(
        isZeroQ(
          makePolynomial("x", [
            [1n, sn(0n)],
            [0n, sn(3n)],
          ]),
        ),
      ),
    ).toBe("false");
  });

  it("sees a polynomial of zero coefficients as a zero coefficient", () => {
    const zeroInY = makePolynomial("y", [[1n, sn(0n)]]);
    expect(show(isZeroQ(makePolynomial("x", [[1n, zeroInY]])))).toBe("true");
    const nonzeroInY = makePolynomial("y", [[1n, sn(1n)]]);
    expect(show(isZeroQ(makePolynomial("x", [[1n, nonzeroInY]])))).toBe("false");
  });

  it("lets adjoin-term skip zero polynomial coefficients", () => {
    const zeroInY = makePolynomial("y", []);
    const l = adjoinTerm([3n, zeroInY], []);
    expect(l.length).toBe(0);
  });
});
