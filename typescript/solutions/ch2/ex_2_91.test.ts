// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  div,
  divPoly,
  divTerms,
  makePolynomial,
  makeSchemeNumber,
  type Polynomial,
  remainderTerms,
  show,
  showPoly,
} from "../../packages/ch2/src/05-generic-operations.js";

const sn = (n: bigint) => makeSchemeNumber(n);
const poly = (terms: ReadonlyArray<readonly [bigint, bigint]>) =>
  terms.map(([o, c]) => [o, sn(c)] as const);

const dividend = (): Polynomial =>
  makePolynomial(
    "x",
    poly([
      [5n, 1n],
      [0n, -1n],
    ]),
  );
const divisor = (): Polynomial =>
  makePolynomial(
    "x",
    poly([
      [2n, 1n],
      [0n, -1n],
    ]),
  );

describe("exercise 2.91: polynomial division", () => {
  it("divides x^5 - 1 by x^2 - 1, the book's example", () => {
    expect(show(div(dividend(), divisor()))).toBe(
      "(quotient-remainder (polynomial x (3 1) (1 1)) (polynomial x (1 1) (0 -1)))",
    );
  });

  it("div-poly answers the quotient and remainder polys in order", () => {
    const qr = divPoly(
      {
        variable: "x",
        terms: poly([
          [5n, 1n],
          [0n, -1n],
        ]),
      },
      {
        variable: "x",
        terms: poly([
          [2n, 1n],
          [0n, -1n],
        ]),
      },
    );
    expect(qr._tag === "Ok" && qr.value[0]).toEqual({
      variable: "x",
      terms: poly([
        [3n, 1n],
        [1n, 1n],
      ]),
    });
    expect(qr._tag === "Ok" && qr.value[1]).toEqual({
      variable: "x",
      terms: poly([
        [1n, 1n],
        [0n, -1n],
      ]),
    });
  });

  it("remainder-terms picks out the remainder component", () => {
    const r = remainderTerms(
      poly([
        [5n, 1n],
        [0n, -1n],
      ]),
      poly([
        [2n, 1n],
        [0n, -1n],
      ]),
    );
    expect(r._tag === "Ok" && showPoly({ variable: "x", terms: r.value })).toBe(
      "(polynomial x (1 1) (0 -1))",
    );
  });

  it("answers an empty quotient when the divisor outranks the dividend", () => {
    const qr = divTerms(poly([[1n, 1n]]), poly([[2n, 1n]]));
    expect(qr._tag === "Ok" && qr.value[0].length === 0 && qr.value[1].length === 1).toBe(true);
  });
});
