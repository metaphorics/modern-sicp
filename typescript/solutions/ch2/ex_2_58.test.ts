// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  deriv,
  qlist,
  qnum,
  qsym,
  showExpr,
  sym,
} from "../../packages/ch2/src/03-symbolic-data.js";
import { parseInfix, parseInfixStandard } from "./ex_2_58.js";

describe("exercise 2.58", () => {
  it("a. parses the fully parenthesized infix form", () => {
    const infixA = qlist(
      qsym("x"),
      qsym("+"),
      qlist(qnum(3), qsym("*"), qlist(qsym("x"), qsym("+"), qlist(qsym("y"), qsym("+"), qnum(2)))),
    );
    const e = parseInfix(infixA);
    expect(showExpr(deriv(e, sym("x")))).toBe("4");
  });

  it("b. parses the standard precedence notation", () => {
    const infixB = qlist(
      qsym("x"),
      qsym("+"),
      qnum(3),
      qsym("*"),
      qlist(qsym("x"), qsym("+"), qsym("y"), qsym("+"), qnum(2)),
    );
    const e = parseInfixStandard(infixB);
    expect(showExpr(e)).toBe("(+ x (* 3 (+ (+ x y) 2)))");
    expect(showExpr(deriv(e, sym("x")))).toBe("4");
  });
});
