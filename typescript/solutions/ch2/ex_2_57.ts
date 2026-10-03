// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type Symb, sym } from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.57: n-ary sums and products. The union's `Sum` and `Prod`
 * shapes widen to carry a list of two or more terms, and the selectors
 * keep the book's meaning: the addend is the first term, the augend is
 * the sum of the rest; the multiplier the first factor, the
 * multiplicand the product of the rest. The simplifying constructors
 * fold numeric terms and absorb 0 and 1, so the differentiation switch
 * keeps its book shape.
 */
export type ExprN =
  | { readonly _tag: "Var"; readonly name: Symb }
  | { readonly _tag: "Num"; readonly n: number }
  | { readonly _tag: "SumN"; readonly terms: ReadonlyArray<ExprN> }
  | { readonly _tag: "ProdN"; readonly terms: ReadonlyArray<ExprN> };

/** Builds the variable named `name`. */
export const varN = (name: string): ExprN => ({ _tag: "Var", name: sym(name) });

/** Builds the constant `n`. */
export const numN = (n: number): ExprN => ({ _tag: "Num", n });

/** Builds the sum of two or more terms: the book's `make-sum` widened. */
export const sumN = (...terms: ReadonlyArray<ExprN>): ExprN => ({ _tag: "SumN", terms });

/** Builds the product of two or more terms: the book's `make-product` widened. */
export const prodN = (...terms: ReadonlyArray<ExprN>): ExprN => ({ _tag: "ProdN", terms });

/** The addend of an n-ary sum: its first term. */
export const addendN = (s: ExprN): ExprN => {
  if (s._tag !== "SumN" || s.terms.length === 0) {
    throw new Error("addendN of a non-sum");
  }
  return s.terms[0] ?? numN(0);
};

/** The augend of an n-ary sum: the sum of the rest of the terms. */
export const augendN = (s: ExprN): ExprN => {
  if (s._tag !== "SumN" || s.terms.length === 0) {
    throw new Error("augendN of a non-sum");
  }
  return makeSumN(...s.terms.slice(1));
};

/** The multiplier of an n-ary product: its first factor. */
export const multiplierN = (p: ExprN): ExprN => {
  if (p._tag !== "ProdN" || p.terms.length === 0) {
    throw new Error("multiplierN of a non-product");
  }
  return p.terms[0] ?? numN(1);
};

/** The multiplicand of an n-ary product: the product of the rest. */
export const multiplicandN = (p: ExprN): ExprN => {
  if (p._tag !== "ProdN" || p.terms.length === 0) {
    throw new Error("multiplicandN of a non-product");
  }
  return makeProductN(...p.terms.slice(1));
};

const addInto = (kept: ExprN[], total: number, t: ExprN): number => {
  if (t._tag === "Num") {
    return total + t.n;
  }
  if (t._tag === "SumN") {
    for (const u of t.terms) {
      total = addInto(kept, total, u);
    }
    return total;
  }
  kept.push(t);
  return total;
};

/** The simplifying n-ary `make-sum`: folds numbers, flattens nested
 * sums, drops zeros, and collapses to the single term when one is left. */
export const makeSumN = (...terms: ReadonlyArray<ExprN>): ExprN => {
  const kept: ExprN[] = [];
  let total = 0;
  for (const t of terms) {
    total = addInto(kept, total, t);
  }
  const all = total === 0 ? kept : [...kept, numN(total)];
  const [first, ...rest] = all;
  if (first === undefined) {
    return numN(0);
  }
  return rest.length === 0 ? first : { _tag: "SumN", terms: all };
};

const mulInto = (kept: ExprN[], total: number, t: ExprN): number => {
  if (t._tag === "Num") {
    return total * t.n;
  }
  if (t._tag === "ProdN") {
    for (const u of t.terms) {
      total = mulInto(kept, total, u);
    }
    return total;
  }
  kept.push(t);
  return total;
};

/** The simplifying n-ary `make-product`: folds numbers, flattens
 * nested products, absorbs 0 and 1, and collapses single factors. */
export const makeProductN = (...terms: ReadonlyArray<ExprN>): ExprN => {
  const kept: ExprN[] = [];
  let total = 1;
  for (const t of terms) {
    total = mulInto(kept, total, t);
  }
  if (total === 0) {
    return numN(0);
  }
  const all = total === 1 ? kept : [...kept, numN(total)];
  const [first, ...rest] = all;
  if (first === undefined) {
    return numN(1);
  }
  return rest.length === 0 ? first : { _tag: "ProdN", terms: all };
};

/** Renders an n-ary expression with minimal-precedence infix notation. */
export const showExprN = (e: ExprN): string => renderExprN(e, 0, "root");

type NSide = "root" | "left" | "right";

const precedenceN = (e: ExprN): number => (e._tag === "SumN" ? 1 : e._tag === "ProdN" ? 2 : 3);

const renderExprN = (e: ExprN, parentPrecedence: number, side: NSide): string => {
  const precedence = precedenceN(e);
  const text = (() => {
    switch (e._tag) {
      case "Var":
        return e.name;
      case "Num":
        return String(e.n);
      case "SumN":
        return e.terms.map((term) => renderExprN(term, 1, "left")).join(" + ");
      case "ProdN":
        return e.terms.map((term) => renderExprN(term, 2, "left")).join(" * ");
    }
  })();
  return precedence < parentPrecedence || (side === "right" && precedence === parentPrecedence)
    ? `(${text})`
    : text;
};

/** The differentiation switch, unchanged in shape: sums differentiate
 * term by term; a product differentiates one factor at a time against
 * the product of the rest. */
export const derivN = (exp: ExprN, v: Symb): ExprN => {
  switch (exp._tag) {
    case "Num":
      return numN(0);
    case "Var":
      return exp.name === v ? numN(1) : numN(0);
    case "SumN":
      return makeSumN(...exp.terms.map((t) => derivN(t, v)));
    case "ProdN":
      return makeSumN(
        ...exp.terms.map((t, i) =>
          makeProductN(derivN(t, v), ...exp.terms.filter((_, j) => j !== i)),
        ),
      );
  }
};
