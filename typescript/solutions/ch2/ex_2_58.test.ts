// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  deriv,
  listDatum,
  numDatum,
  showExpr,
  sym,
  symDatum,
} from "../../packages/ch2/src/03-symbolic-data.js";
import { parseInfix, parseInfixStandard } from "./ex_2_58.js";

describe("exercise 2.58", () => {
  it("a. parses the fully parenthesized infix form", () => {
    const infixA = listDatum(
      symDatum("x"),
      symDatum("+"),
      listDatum(
        numDatum(3),
        symDatum("*"),
        listDatum(
          symDatum("x"),
          symDatum("+"),
          listDatum(symDatum("y"), symDatum("+"), numDatum(2)),
        ),
      ),
    );
    const e = parseInfix(infixA);
    expect(showExpr(deriv(e, sym("x")))).toBe("4");
  });

  it("b. parses the standard precedence notation", () => {
    const infixB = listDatum(
      symDatum("x"),
      symDatum("+"),
      numDatum(3),
      symDatum("*"),
      listDatum(symDatum("x"), symDatum("+"), symDatum("y"), symDatum("+"), numDatum(2)),
    );
    const e = parseInfixStandard(infixB);
    expect(showExpr(e)).toBe("x + 3 * (x + y + 2)");
    expect(showExpr(deriv(e, sym("x")))).toBe("4");
  });
});
