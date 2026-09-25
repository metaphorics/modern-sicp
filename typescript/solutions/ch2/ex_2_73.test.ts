// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import type { Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { sym } from "../../packages/ch2/src/03-symbolic-data.js";
import {
  buildPow,
  buildProduct,
  buildSum,
  constant,
  type DerivError,
  derivDataDirected,
  derivFlipped,
  derivTable,
  derivTableFlipped,
  derivToString,
  type Expr,
  operatorOf,
  showDerivError,
  showDerivExpr,
  variable,
} from "./ex_2_73.js";

const shown = (r: Result<Expr, DerivError>): string =>
  r._tag === "Ok" ? showDerivExpr(r.value) : showDerivError(r.error);

const x = variable("x");
const y = variable("y");
const dx = sym("x");

describe("exercise 2.73: data-directed deriv", () => {
  it("answers the book's derivatives through the table", () => {
    expect(derivToString(x, dx)).toEqual({ _tag: "Ok", value: "1" });
    expect(derivToString(buildProduct(x, y), dx)).toEqual({ _tag: "Ok", value: "y" });
    expect(derivToString(buildSum(x, constant(3)), dx)).toEqual({ _tag: "Ok", value: "1" });
  });

  it("part a keeps the constant and variable cases residual", () => {
    expect(derivToString(constant(5), dx)).toEqual({ _tag: "Ok", value: "0" });
    expect(derivToString(variable("y"), dx)).toEqual({ _tag: "Ok", value: "0" });
    // The table keys operators, so the residual shapes have no entry to
    // be assimilated into; the error names the book's message.
    expect(showDerivError({ _tag: "UnknownExpressionType", op: "+" })).toBe(
      "unknown expression type: DERIV +",
    );
  });

  it("part b installs sum and product rules that simplify", () => {
    const sum = buildSum(x, x);
    const product = buildProduct(x, y);
    expect(sum._tag === "Sum" ? operatorOf(sum) : "not compound").toBe("+");
    expect(product._tag === "Prod" ? operatorOf(product) : "not compound").toBe("*");
    // d(3 * x^2) = 0 * x^2 + 3 * d(x^2), simplified by the 2.3.2
    // constructors to the printed form.
    expect(derivToString(buildProduct(constant(3), buildPow(x, constant(2))), dx)).toEqual({
      _tag: "Ok",
      value: "(* 3 (* 2 (** x 1)))",
    });
  });

  it("part c installs the exponentiation rule of exercise 2.56", () => {
    expect(derivToString(buildPow(x, constant(3)), dx)).toEqual({
      _tag: "Ok",
      value: "(* 3 (** x 2))",
    });
  });

  it("an operator with no rule reports the book's error", () => {
    // d(x^x) needs u^n with symbolic n; no rule is installed, so the
    // lookup, not a missing match arm, reports the failure.
    const d = derivDataDirected(buildPow(x, x), dx);
    expect(d._tag).toBe("Error");
    if (d._tag === "Error") {
      expect(showDerivError(d.error)).toBe("unknown expression type: DERIV **");
    }
  });

  it("part d: flipped indexing answers the same questions", () => {
    expect(shown(derivFlipped(buildSum(x, constant(3)), dx))).toBe("1");
    expect(shown(derivFlipped(buildProduct(x, y), dx))).toBe("y");
    expect(shown(derivFlipped(buildPow(x, constant(3)), dx))).toBe("(* 3 (** x 2))");
    // Only the index changed, not the rules: the unflipped table keys
    // the operation first, the flipped one the operator.
    expect(derivTable.get("deriv")?.get("+")).toBeDefined();
    expect(derivTableFlipped.get("deriv")).toBeUndefined();
    expect(derivTableFlipped.get("+")?.get("deriv")).toBeDefined();
  });
});
