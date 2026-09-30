// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import type { Option } from "../../packages/ch2/src/02-picture-language.js";
import { type Symb, sym } from "../../packages/ch2/src/03-symbolic-data.js";
import { get, makeOpTable, type OpTable, put } from "../../packages/ch2/src/04-data-directed.js";

/**
 * Exercise 2.73: the section 2.3.2 `deriv` program, rebuilt so the
 * dispatch lives in an operation-and-type table. The expression union's
 * operator plays the type tag: each rule is installed under the
 * operation `deriv` with that operator as the type, and
 * `derivDataDirected` looks rules up
 * instead of switching on them. Parts (a) and (d) are answered by the
 * residual arms here and in the test; part (c) installs the
 * exponentiation rule of exercise 2.56 under the `**` operator.
 */

/** An expression for this exercise: the section 2.3.2 union extended
 * with the `**` operator that part (c) installs a rule for. */
export type Expr =
  | { readonly _tag: "Num"; readonly n: number }
  | { readonly _tag: "Var"; readonly name: Symb }
  | { readonly _tag: "Sum"; readonly addend: Expr; readonly augend: Expr }
  | { readonly _tag: "Prod"; readonly multiplier: Expr; readonly multiplicand: Expr }
  | { readonly _tag: "Pow"; readonly base: Expr; readonly exp: Expr };

/** Builds the variable named `name`: the book's `'x`. */
export const variable = (name: string): Expr => ({ _tag: "Var", name: sym(name) });

/** Builds the numeric constant `n`. */
export const constant = (n: number): Expr => ({ _tag: "Num", n });

/** Builds the sum `(a + b)`; the simplifying constructors from 2.3.2
 * shape the results, exactly as the section's deriv did. */
export const buildSum = (a: Expr, b: Expr): Expr => ({ _tag: "Sum", addend: a, augend: b });

/** Builds the product `(a * b)`. */
export const buildProduct = (a: Expr, b: Expr): Expr => ({
  _tag: "Prod",
  multiplier: a,
  multiplicand: b,
});

/** Builds the power `(base ** exp)`. */
export const buildPow = (base: Expr, exp: Expr): Expr => ({ _tag: "Pow", base, exp });

/** Tests whether an expression is the constant `n`: the host's equality
 * test against the number literal. */
const equalsNumber = (e: Expr, n: number): boolean => e._tag === "Num" && e.n === n;

/** Tests whether an expression is the variable `v`: the name equality
 * after the tag check. */
const sameVariableQ = (e: Expr, v: Symb): boolean => e._tag === "Var" && e.name === v;

/** The book's revised make-sum: adds two constants, absorbs 0. */
export const makeSum = (a1: Expr, a2: Expr): Expr => {
  if (equalsNumber(a1, 0)) {
    return a2;
  }
  if (equalsNumber(a2, 0)) {
    return a1;
  }
  if (a1._tag === "Num" && a2._tag === "Num") {
    return constant(a1.n + a2.n);
  }
  return buildSum(a1, a2);
};

/** The book's revised make-product: absorbs 0 and 1, multiplies two
 * constants. */
export const makeProduct = (m1: Expr, m2: Expr): Expr => {
  if (equalsNumber(m1, 0) || equalsNumber(m2, 0)) {
    return constant(0);
  }
  if (equalsNumber(m1, 1)) {
    return m2;
  }
  if (equalsNumber(m2, 1)) {
    return m1;
  }
  if (m1._tag === "Num" && m2._tag === "Num") {
    return constant(m1.n * m2.n);
  }
  return buildProduct(m1, m2);
};

/** A compound expression: the variants the table indexes by operator. */
export type CompoundExpr = Extract<Expr, { _tag: "Sum" | "Prod" | "Pow" }>;

/** The book's operator: the algebraic operator that binds the
 * expression, the datum's type tag here. */
export const operatorOf = (exp: CompoundExpr): string =>
  exp._tag === "Sum" ? "+" : exp._tag === "Prod" ? "*" : "**";

/** The book's operands: the expressions the operator binds, in order. */
export const operandsOf = (exp: CompoundExpr): ReadonlyArray<Expr> => {
  switch (exp._tag) {
    case "Sum":
      return [exp.addend, exp.augend];
    case "Prod":
      return [exp.multiplier, exp.multiplicand];
    case "Pow":
      return [exp.base, exp.exp];
  }
};

/** Renders an expression with minimal-precedence infix notation. */
export const showDerivExpr = (e: Expr): string => renderDerivExpr(e, 0, "root");

type DerivSide = "root" | "left" | "right";

const derivPrecedence = (e: Expr): number =>
  e._tag === "Sum" ? 1 : e._tag === "Prod" ? 2 : e._tag === "Pow" ? 3 : 4;

const renderDerivExpr = (e: Expr, parentPrecedence: number, side: DerivSide): string => {
  const precedence = derivPrecedence(e);
  const text = (() => {
    switch (e._tag) {
      case "Var":
        return e.name;
      case "Num":
        return String(e.n);
      case "Sum":
        return `${renderDerivExpr(e.addend, 1, "left")} + ${renderDerivExpr(e.augend, 1, "right")}`;
      case "Prod":
        return `${renderDerivExpr(e.multiplier, 2, "left")} * ${renderDerivExpr(e.multiplicand, 2, "right")}`;
      case "Pow":
        return `${renderDerivExpr(e.base, 3, "left")} ** ${renderDerivExpr(e.exp, 3, "right")}`;
    }
  })();
  return precedence < parentPrecedence || (side === "right" && precedence === parentPrecedence)
    ? `(${text})`
    : text;
};

/** Why a deriv lookup cannot answer: the book's "unknown expression
 * type -- DERIV", carrying the operator that had no rule. */
export type DerivError = { readonly _tag: "UnknownExpressionType"; readonly op: string };

/** The book's error line, rendered. */
export const showDerivError = (e: DerivError): string => `deriv: unknown expression type ${e.op}`;

/** A rule: the operands of one operator plus the variable of
 * differentiation, answering the differentiated expression. */
export type DerivRule = (operands: ReadonlyArray<Expr>, v: Symb) => Result<Expr, DerivError>;

/** This exercise's operation-and-type table, keyed `deriv` by operator. */
export type DerivTable = OpTable<DerivRule>;

/** Installs a rule for `op` in `table`. */
export const installDerivRule = (table: DerivTable, op: string, rule: DerivRule): void => {
  put(table, "deriv", [op], rule);
};

/** Combines two differentiated operands with the simplifying
 * constructor `f`, propagating a failed subderivation. */
const both = (
  r1: Result<Expr, DerivError>,
  r2: Result<Expr, DerivError>,
  f: (a: Expr, b: Expr) => Expr,
): Result<Expr, DerivError> =>
  r1._tag === "Error" ? r1 : r2._tag === "Error" ? r2 : ok(f(r1.value, r2.value));

/** The table of this exercise's answer, the sum and product rules of
 * part (b) and the power rule of part (c) already installed. */
export const derivTable: DerivTable = makeOpTable();

// Part (b): the sum and product rules, written against the simplifying
// constructors the section ended with. Missing operands cannot occur in
// a well-formed expression; the arms return the book's error if one
// ever did.
installDerivRule(derivTable, "+", (operands, v) => {
  const [a1, a2] = operands;
  if (a1 === undefined || a2 === undefined) {
    return err({ _tag: "UnknownExpressionType", op: "+" });
  }
  return both(derivDataDirected(a1, v), derivDataDirected(a2, v), makeSum);
});
installDerivRule(derivTable, "*", (operands, v) => {
  const [m1, m2] = operands;
  if (m1 === undefined || m2 === undefined) {
    return err({ _tag: "UnknownExpressionType", op: "*" });
  }
  const dMultiplier = derivDataDirected(m1, v);
  const dMultiplicand = derivDataDirected(m2, v);
  return both(dMultiplier, dMultiplicand, (dm, dn) =>
    makeSum(makeProduct(m1, dn), makeProduct(dm, m2)),
  );
});

// Part (c): the exponentiation rule of exercise 2.56, installed like
// any other: du^n = n * u^(n-1) * du, for numeric n.
installDerivRule(derivTable, "**", (operands, v) => {
  const [base, exp] = operands;
  if (base === undefined || exp === undefined || exp._tag !== "Num") {
    return err({ _tag: "UnknownExpressionType", op: "**" });
  }
  const lowered = buildPow(base, constant(exp.n - 1));
  const dBase = derivDataDirected(base, v);
  return both(ok(constant(exp.n)), dBase, (n, du) => makeProduct(n, makeProduct(lowered, du)));
});

/** The data-directed `deriv`: the constant and variable cases stay
 * residual (part a), everything else is one table lookup. */
export const derivDataDirected = (exp: Expr, v: Symb): Result<Expr, DerivError> => {
  if (exp._tag === "Num") {
    return ok(constant(0));
  }
  if (exp._tag === "Var") {
    return ok(sameVariableQ(exp, v) ? constant(1) : constant(0));
  }
  const rule = get(derivTable, "deriv", [operatorOf(exp)]);
  if (rule._tag === "None") {
    return err({ _tag: "UnknownExpressionType", op: operatorOf(exp) });
  }
  return rule.value(operandsOf(exp), v);
};

/** Part (d): a table indexed the other way round, operator first and
 * the operation name as the tag, per the book's flipped dispatch line. */
export const derivTableFlipped: DerivTable = makeOpTable();

const mirrorRules = (): void => {
  for (const op of ["+", "*", "**"]) {
    const rule = get(derivTable, "deriv", [op]);
    if (rule._tag === "Some") {
      put(derivTableFlipped, op, ["deriv"], rule.value);
    }
  }
};
mirrorRules();

/** The flipped dispatch: looks the rule up by operator first. */
export const derivFlipped = (exp: Expr, v: Symb): Result<Expr, DerivError> => {
  if (exp._tag === "Num") {
    return ok(constant(0));
  }
  if (exp._tag === "Var") {
    return ok(sameVariableQ(exp, v) ? constant(1) : constant(0));
  }
  const rule: Option<DerivRule> = get(derivTableFlipped, operatorOf(exp), ["deriv"]);
  if (rule._tag === "None") {
    return err({ _tag: "UnknownExpressionType", op: operatorOf(exp) });
  }
  return rule.value(operandsOf(exp), v);
};

/** The derivative as a printable string, for the asserted answers. */
export const derivToString = (exp: Expr, v: Symb): Result<string, DerivError> => {
  const d = derivDataDirected(exp, v);
  return d._tag === "Ok" ? ok(showDerivExpr(d.value)) : d;
};
