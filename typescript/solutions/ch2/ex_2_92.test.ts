// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  makePolynomial,
  makeSchemeNumber,
  show,
} from "../../packages/ch2/src/05-generic-operations.js";
import { addMulti, mulMulti, poly92, polyMixed92 } from "./ex_2_92.js";

describe("exercise 2.92: polynomials in different variables", () => {
  it("adds polys in different variables by promoting the lower one", () => {
    const sum = addMulti(poly92("x", [[1n, 5n]]), poly92("y", [[1n, 1n]]));
    expect(show(sum)).toBe("(polynomial x (1 5) (0 (polynomial y (1 1))))");
  });

  it("multiplies polys in different variables", () => {
    const product = mulMulti(poly92("x", [[1n, 1n]]), poly92("y", [[1n, 1n]]));
    expect(show(product)).toBe("(polynomial x (1 (polynomial y (1 1))))");
  });

  it("lifts a bare coefficient onto the other coefficient's variable", () => {
    // (y + 1) x^2 + 5 plus x^2 + 2x + 1: the footnote's coercion.
    const p1 = polyMixed92("x", [
      [
        2n,
        makePolynomial(
          "y",
          poly92("y", [
            [1n, 1n],
            [0n, 1n],
          ]).terms,
        ),
      ],
      [0n, makeSchemeNumber(5n)],
    ]);
    const p2 = poly92("x", [
      [2n, 1n],
      [1n, 2n],
      [0n, 1n],
    ]);
    expect(show(addMulti(p1, p2))).toBe(
      "(polynomial x (2 (polynomial y (1 1) (0 2))) (1 2) (0 6))",
    );
  });

  it("still adds same-variable polys through the ordered merge", () => {
    const sum = addMulti(
      poly92("x", [
        [2n, 1n],
        [0n, 5n],
      ]),
      poly92("x", [
        [2n, 1n],
        [1n, 2n],
        [0n, 1n],
      ]),
    );
    expect(show(sum)).toBe("(polynomial x (2 2) (1 2) (0 6))");
  });
});
