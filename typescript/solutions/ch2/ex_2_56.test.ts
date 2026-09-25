// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { sym } from "../../packages/ch2/src/03-symbolic-data.js";
import { derivPow, makeExponentiation, numPow, showExprPow, varPow } from "./ex_2_56.js";

describe("exercise 2.56", () => {
  it("differentiates u^n by the power rule", () => {
    expect(showExprPow(derivPow(makeExponentiation(varPow("x"), numPow(2)), sym("x")))).toBe(
      "(* 2 x)",
    );
  });

  it("builds in the power-0 and power-1 rules", () => {
    expect(showExprPow(makeExponentiation(varPow("u"), numPow(0)))).toBe("1");
    expect(showExprPow(makeExponentiation(varPow("u"), numPow(1)))).toBe("u");
  });

  it("keeps symbolic exponents and folds constants in the result", () => {
    expect(showExprPow(derivPow(makeExponentiation(varPow("x"), numPow(3)), sym("x")))).toBe(
      "(* 3 (** x 2))",
    );
    expect(showExprPow(derivPow(makeExponentiation(varPow("y"), numPow(2)), sym("x")))).toBe("0");
  });
});
