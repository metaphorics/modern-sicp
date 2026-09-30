// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { sym } from "../../packages/ch2/src/03-symbolic-data.js";
import {
  addendN,
  augendN,
  derivN,
  makeProductN,
  makeSumN,
  numN,
  prodN,
  showExprN,
  sumN,
  varN,
} from "./ex_2_57.js";

describe("exercise 2.57", () => {
  it("differentiates the book's n-ary example", () => {
    const e = prodN(varN("x"), varN("y"), sumN(varN("x"), numN(3)));
    expect(showExprN(derivN(e, sym("x")))).toBe("y * (x + 3) + x * y");
  });

  it("the selectors keep the book's addend/augend split", () => {
    const s = sumN(varN("x"), varN("y"), numN(3));
    expect(showExprN(addendN(s))).toBe("x");
    expect(showExprN(augendN(s))).toBe("y + 3");
    expect(showExprN(makeSumN(addendN(s), augendN(s)))).toBe("x + y + 3");
  });

  it("the simplifying constructors fold, flatten, and absorb", () => {
    expect(showExprN(makeSumN(numN(2), numN(3), varN("z")))).toBe("z + 5");
    expect(showExprN(makeSumN(numN(0), varN("z")))).toBe("z");
    expect(showExprN(makeProductN(numN(1), varN("z"), sumN(varN("x"), numN(2))))).toBe(
      "z * (x + 2)",
    );
    expect(showExprN(makeProductN(numN(0), varN("z")))).toBe("0");
    expect(showExprN(makeProductN(makeProductN(numN(2), varN("a")), numN(3)))).toBe("a * 6");
  });
});
