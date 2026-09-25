// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  add,
  equQ,
  isZeroQ,
  makeRational,
  show,
} from "../../packages/ch2/src/05-generic-operations.js";
import { attachTagSchemeNumber, contents78, typeTag78 } from "./ex_2_78.js";

describe("exercise 2.78: primitive type tags", () => {
  it("a bare number names itself scheme-number", () => {
    expect(typeTag78(7n)).toBe("scheme-number");
    expect(typeTag78(attachTagSchemeNumber(7n))).toBe("scheme-number");
    expect(typeTag78(makeRational(1n, 2n))).toBe("rational");
  });

  it("a bare number is its own contents", () => {
    expect(contents78(7n)).toBe(7n);
    expect(contents78(attachTagSchemeNumber(7n))).toBe(7n);
    expect(contents78(makeRational(1n, 2n))).toEqual([1n, 2n]);
  });

  it("the tower operates on bare numbers with no wrapper", () => {
    expect(attachTagSchemeNumber(7n)).toBe(7n);
    expect(show(add(attachTagSchemeNumber(3n), attachTagSchemeNumber(4n)))).toBe("7");
    expect(show(equQ(attachTagSchemeNumber(3n), attachTagSchemeNumber(3n)))).toBe("true");
    expect(show(isZeroQ(attachTagSchemeNumber(0n)))).toBe("true");
  });
});
