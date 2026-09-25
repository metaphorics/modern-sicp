// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 2.5

import { describe, expect, it } from "vitest";

import {
  add,
  adjoinTerm,
  div,
  divPoly,
  equQ,
  exp,
  greatestCommonDivisor,
  imagPart,
  isZeroQ,
  magnitude,
  makeComplexFromMagAng,
  makeComplexFromRealImag,
  makePolynomial,
  makeRational,
  makeSchemeNumber,
  mul,
  mulTerms,
  realPart,
  show,
  sub,
} from "./05-generic-operations.js";

const sn = (n: bigint): bigint => makeSchemeNumber(n);
const rat = (n: bigint, d: bigint) => makeRational(n, d);
const cpx = (x: number, y: number) => makeComplexFromRealImag(x, y);
const poly = (v: string, terms: ReadonlyArray<readonly [bigint, bigint]>) =>
  makePolynomial(
    v,
    terms.map(([o, c]) => [o, sn(c)] as const),
  );

describe("section 2.5: systems with generic operations", () => {
  it("2.5.1 dispatches add, sub, mul, and div through the table", () => {
    expect(show(add(sn(3n), sn(4n)))).toBe("7");
    expect(show(sub(sn(3n), sn(4n)))).toBe("-1");
    expect(show(add(rat(1n, 2n), rat(1n, 3n)))).toBe("(rational 5 6)");
    expect(show(mul(rat(2n, 3n), rat(3n, 4n)))).toBe("(rational 1 2)");
    expect(show(div(rat(1n, 2n), rat(1n, 3n)))).toBe("(rational 3 2)");
    expect(show(add(cpx(1, 2), cpx(3, 4)))).toBe("(complex rectangular 4 6)");
    expect(show(sub(cpx(3, 4), cpx(1, 2)))).toBe("(complex rectangular 2 2)");
    expect(show(mul(cpx(1, 2), cpx(3, 4)))).toBe(
      "(complex polar 11.180339887498949 2.0344439357957027)",
    );
    expect(show(div(cpx(1, 2), cpx(3, 4)))).toBe(
      "(complex polar 0.447213595499958 0.17985349979247822)",
    );
  });

  it("2.5.1 builds every kind of number through the table's constructors", () => {
    expect(makeSchemeNumber(7n)).toBe(7n);
    expect(rat(3n, 6n)).toEqual({ _tag: "rational", contents: [1n, 2n] });
    expect(cpx(3, 4)).toEqual({
      _tag: "complex",
      contents: { _tag: "rectangular", contents: [3, 4] },
    });
    expect(makeComplexFromMagAng(5, 0.9272952180016122)).toEqual({
      _tag: "complex",
      contents: { _tag: "polar", contents: [5, 0.9272952180016122] },
    });
  });

  it("2.5.1 answers the complex selectors one tag down, as 2.77 requires", () => {
    expect(show(realPart(cpx(1, 2)))).toBe("1");
    expect(show(imagPart(cpx(1, 2)))).toBe("2");
    expect(show(magnitude(cpx(3, 4)))).toBe("5");
    expect(show(magnitude(makeComplexFromMagAng(2, 1.5707963267948966)))).toBe("2");
    expect(show(magnitude(sn(3n)))).toBe(
      "No method for these types: APPLY-GENERIC (magnitude (scheme-number))",
    );
  });

  it("2.5.2 combines complex and ordinary numbers by coercion", () => {
    expect(show(add(cpx(1, 2), sn(4n)))).toBe("(complex rectangular 5 2)");
    expect(show(mul(sn(2n), cpx(3, 4)))).toBe("(complex polar 10 0.9272952180016122)");
    expect(show(exp(cpx(1, 2), cpx(3, 4)))).toBe(
      "No method for these types: APPLY-GENERIC (exp (complex complex))",
    );
    expect(show(exp(sn(2n), sn(10n)))).toBe("1024");
  });

  it("2.5.2 answers the 2.79 and 2.80 predicates at every level", () => {
    expect(show(equQ(sn(3n), sn(3n)))).toBe("true");
    expect(show(equQ(sn(3n), sn(4n)))).toBe("false");
    expect(show(equQ(rat(1n, 2n), rat(2n, 4n)))).toBe("true");
    expect(show(equQ(cpx(1, 2), cpx(1, 2)))).toBe("true");
    expect(show(isZeroQ(sn(0n)))).toBe("true");
    expect(show(isZeroQ(sn(5n)))).toBe("false");
    expect(show(isZeroQ(rat(0n, 5n)))).toBe("true");
    expect(show(isZeroQ(cpx(0, 0)))).toBe("true");
  });

  it("2.5.3 adds and multiplies polynomials termwise", () => {
    const p = poly("x", [
      [2n, 1n],
      [1n, -2n],
      [0n, 1n],
    ]);
    const q = poly("x", [
      [1n, 1n],
      [0n, 3n],
    ]);
    expect(show(add(p, q))).toBe("(polynomial x (2 1) (1 -1) (0 4))");
    expect(show(mul(p, q))).toBe("(polynomial x (3 1) (2 1) (1 -5) (0 3))");
  });

  it("2.5.3 recurses data-directedly through polynomial coefficients", () => {
    const inY = (terms: ReadonlyArray<readonly [bigint, bigint]>) => poly("y", terms);
    const p = makePolynomial("x", [
      [
        2n,
        inY([
          [1n, 1n],
          [0n, 1n],
        ]),
      ],
      [
        1n,
        inY([
          [2n, 1n],
          [0n, 1n],
        ]),
      ],
      [
        0n,
        inY([
          [1n, 1n],
          [0n, -1n],
        ]),
      ],
    ]);
    const q = makePolynomial("x", [
      [
        1n,
        inY([
          [1n, 1n],
          [0n, -2n],
        ]),
      ],
      [
        0n,
        inY([
          [3n, 1n],
          [0n, 7n],
        ]),
      ],
    ]);
    expect(show(mul(p, q))).toBe(
      "(polynomial x (3 (polynomial y (2 1) (1 -1) (0 -2))) (2 (polynomial y (4 1) (3 2) (2 -2) (1 8) (0 5))) (1 (polynomial y (5 1) (3 1) (2 8) (1 -3) (0 9))) (0 (polynomial y (4 1) (3 -1) (1 7) (0 -7))))",
    );
  });

  it("2.5.3 reports polys not in the same var", () => {
    const inX = poly("x", [[1n, 1n]]);
    const inY = poly("y", [[1n, 1n]]);
    expect(show(add(inX, inY))).toBe(
      "Polys not in same var: ADD-POLY ((polynomial x (1 1)) (polynomial y (1 1)))",
    );
  });

  it("2.5.3 divides polynomials by long division, as 2.91 fills in", () => {
    const qr = div(
      poly("x", [
        [5n, 1n],
        [0n, -1n],
      ]),
      poly("x", [
        [2n, 1n],
        [0n, -1n],
      ]),
    );
    expect(show(qr)).toBe(
      "(quotient-remainder (polynomial x (3 1) (1 1)) (polynomial x (1 1) (0 -1)))",
    );
    expect(
      divPoly(
        poly("x", [
          [5n, 1n],
          [0n, -1n],
        ]).contents,
        poly("x", [
          [2n, 1n],
          [0n, -1n],
        ]).contents,
      ),
    ).toEqual({
      _tag: "Ok",
      value: [
        {
          variable: "x",
          terms: [
            [3n, sn(1n)],
            [1n, sn(1n)],
          ],
        },
        {
          variable: "x",
          terms: [
            [1n, sn(1n)],
            [0n, sn(-1n)],
          ],
        },
      ],
    });
  });

  it("2.5.3 computes the polynomial gcd of 2.94", () => {
    const p1 = poly("x", [
      [4n, 1n],
      [3n, -1n],
      [2n, -2n],
      [1n, 2n],
    ]);
    const p2 = poly("x", [
      [3n, 1n],
      [1n, -1n],
    ]);
    expect(show(greatestCommonDivisor(p1, p2))).toBe("(polynomial x (2 -1) (1 1))");
    expect(show(greatestCommonDivisor(sn(24n), sn(36n)))).toBe("12");
  });

  it("2.5.3 keeps term lists free of zero coefficients", () => {
    expect(adjoinTerm([3n, sn(0n)], []).length).toBe(0);
    const product = mulTerms([], []);
    expect(product._tag === "Ok" && product.value.length === 0).toBe(true);
    expect(show(isZeroQ(makePolynomial("x", [[1n, sn(0n)]])))).toBe("true");
    expect(show(isZeroQ(poly("x", [[1n, 1n]])))).toBe("false");
  });
});
