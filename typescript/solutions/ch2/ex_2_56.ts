// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type Symb, sym } from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.56: exponentiation as a fifth union case. The section's
 * four-case union is redeclared with a `Pow` shape (the book's
 * `(** u n)`), and the differentiation switch gains the one rule
 * d(u^n)/dx = n u^(n-1) du/dx, with the power-0 and power-1 rules
 * folded into `makeExponentiation`.
 */
export type ExprPow =
  | { readonly _tag: "Var"; readonly name: Symb }
  | { readonly _tag: "Num"; readonly n: number }
  | { readonly _tag: "Sum"; readonly addend: ExprPow; readonly augend: ExprPow }
  | { readonly _tag: "Prod"; readonly multiplier: ExprPow; readonly multiplicand: ExprPow }
  | { readonly _tag: "Pow"; readonly base: ExprPow; readonly exponent: ExprPow };

/** Builds the variable named `name`. */
export const varPow = (name: string): ExprPow => ({ _tag: "Var", name: sym(name) });

/** Builds the constant `n`. */
export const numPow = (n: number): ExprPow => ({ _tag: "Num", n });

const equalsNumberPow = (e: ExprPow, n: number): boolean => e._tag === "Num" && e.n === n;

/** The book's `make-sum` with the section's simplifications. */
export const makeSumPow = (a1: ExprPow, a2: ExprPow): ExprPow => {
  if (equalsNumberPow(a1, 0)) {
    return a2;
  }
  if (equalsNumberPow(a2, 0)) {
    return a1;
  }
  if (a1._tag === "Num" && a2._tag === "Num") {
    return numPow(a1.n + a2.n);
  }
  return { _tag: "Sum", addend: a1, augend: a2 };
};

/** The book's `make-product` with the section's simplifications. */
export const makeProductPow = (m1: ExprPow, m2: ExprPow): ExprPow => {
  if (equalsNumberPow(m1, 0) || equalsNumberPow(m2, 0)) {
    return numPow(0);
  }
  if (equalsNumberPow(m1, 1)) {
    return m2;
  }
  if (equalsNumberPow(m2, 1)) {
    return m1;
  }
  if (m1._tag === "Num" && m2._tag === "Num") {
    return numPow(m1.n * m2.n);
  }
  return { _tag: "Prod", multiplier: m1, multiplicand: m2 };
};

/** The book's `make-exponentiation`: u^0 = 1 and u^1 = u are built in. */
export const makeExponentiation = (u: ExprPow, n: ExprPow): ExprPow => {
  if (equalsNumberPow(n, 0)) {
    return numPow(1);
  }
  if (equalsNumberPow(n, 1)) {
    return u;
  }
  return { _tag: "Pow", base: u, exponent: n };
};

/** Renders an expression with minimal-precedence infix notation. */
export const showExprPow = (e: ExprPow): string => renderExprPow(e, 0, "root");

type PowSide = "root" | "left" | "right";

const powPrecedence = (e: ExprPow): number =>
  e._tag === "Sum" ? 1 : e._tag === "Prod" ? 2 : e._tag === "Pow" ? 3 : 4;

const renderExprPow = (e: ExprPow, parentPrecedence: number, side: PowSide): string => {
  const precedence = powPrecedence(e);
  const text = (() => {
    switch (e._tag) {
      case "Var":
        return e.name;
      case "Num":
        return String(e.n);
      case "Sum":
        return `${renderExprPow(e.addend, 1, "left")} + ${renderExprPow(e.augend, 1, "right")}`;
      case "Prod":
        return `${renderExprPow(e.multiplier, 2, "left")} * ${renderExprPow(e.multiplicand, 2, "right")}`;
      case "Pow":
        return `${renderExprPow(e.base, 3, "left")} ** ${renderExprPow(e.exponent, 3, "right")}`;
    }
  })();
  return precedence < parentPrecedence || (side === "right" && precedence === parentPrecedence)
    ? `(${text})`
    : text;
};

/** The extended differentiation switch: the four original arms plus
 * the exponentiation rule. */
export const derivPow = (exp: ExprPow, v: Symb): ExprPow => {
  switch (exp._tag) {
    case "Num":
      return numPow(0);
    case "Var":
      return exp.name === v ? numPow(1) : numPow(0);
    case "Sum":
      return makeSumPow(derivPow(exp.addend, v), derivPow(exp.augend, v));
    case "Prod":
      return makeSumPow(
        makeProductPow(exp.multiplier, derivPow(exp.multiplicand, v)),
        makeProductPow(derivPow(exp.multiplier, v), exp.multiplicand),
      );
    case "Pow":
      return makeProductPow(
        makeProductPow(
          exp.exponent,
          makeExponentiation(exp.base, makeSumPow(exp.exponent, numPow(-1))),
        ),
        derivPow(exp.base, v),
      );
  }
};
