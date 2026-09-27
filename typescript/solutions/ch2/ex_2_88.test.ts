// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  makeComplexFromRealImag,
  makePolynomial,
  makeRational,
  makeSchemeNumber,
  show,
} from "../../packages/ch2/src/05-generic-operations.js";
import { installNegation, negate, poly88, sub88 } from "./ex_2_88.js";

const sn = (n: bigint) => makeSchemeNumber(n);

describe("exercise 2.88: negation and subtraction", () => {
  installNegation();

  it("negates an ordinary number, a rational, and a complex number", () => {
    expect(show(negate(sn(5n)))).toBe("-5");
    expect(show(negate(makeRational(3n, 4n)))).toBe("(rational -3 4)");
    expect(show(negate(makeComplexFromRealImag(1, 2)))).toBe("(complex rectangular -1 -2)");
  });

  it("negates a polynomial, nested coefficients included", () => {
    expect(
      show(
        negate(
          poly88("x", [
            [2n, 1n],
            [1n, -2n],
            [0n, 1n],
          ]),
        ),
      ),
    ).toBe("(polynomial x (2 -1) (1 2) (0 -1))");
    const yTerms: ReadonlyArray<readonly [bigint, bigint]> = [
      [2n, 1n],
      [0n, -1n],
    ];
    const coeffPoly = makePolynomial(
      "y",
      yTerms.map(([o, c]) => [o, sn(c)] as const),
    );
    const outer = makePolynomial("x", [[1n, coeffPoly]]);
    expect(show(negate(outer))).toBe("(polynomial x (1 (polynomial y (2 -1) (0 1))))");
  });

  it("subtracts polynomials by adding the negation", () => {
    expect(
      show(
        sub88(
          poly88("x", [
            [2n, 1n],
            [1n, -2n],
            [0n, 1n],
          ]),
          poly88("x", [
            [1n, 1n],
            [0n, 3n],
          ]),
        ),
      ),
    ).toBe("(polynomial x (2 1) (1 -3) (0 -2))");
  });

  it("subtracts at every level through the same generic operation", () => {
    expect(show(sub88(sn(3n), sn(4n)))).toBe("-1");
    expect(show(sub88(makeRational(1n, 2n), makeRational(1n, 3n)))).toBe("(rational 1 6)");
    expect(show(sub88(makeComplexFromRealImag(1, 2), makeComplexFromRealImag(3, 4)))).toBe(
      "(complex rectangular -2 -2)",
    );
  });
});
