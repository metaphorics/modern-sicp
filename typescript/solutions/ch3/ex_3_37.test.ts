// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeConnector } from "../../packages/ch3/src/03-mutable-data.js";

import { cadd, cconst, cdiv, celsiusFahrenheitConverterExpr, cmul } from "./ex_3_37.js";

describe("exercise 3.37: expression-style constraint combinators", () => {
  it("c +, c *, c /, and cv compose by hand", () => {
    const six = cconst(6);
    const seven = cconst(7);
    const fortyTwo = cmul(six, seven);
    expect(fortyTwo.getValue()).toBe(42);
    const sixAgain = cdiv(fortyTwo, seven);
    expect(sixAgain.getValue()).toBe(6);
    const fortyEight = cadd(fortyTwo, six);
    expect(fortyEight.getValue()).toBe(48);
  });

  it("C = 25 answers F = 77", () => {
    const c = makeConnector();
    const f = celsiusFahrenheitConverterExpr(c);
    c.setValue(25, "user");
    expect(f.getValue()).toBe(77);
  });

  it("C = 100 answers F = 212", () => {
    const c = makeConnector();
    const f = celsiusFahrenheitConverterExpr(c);
    c.setValue(100, "user");
    expect(f.getValue()).toBe(212);
  });

  it("F = 212 answers C = 100, backward through the expression", () => {
    const c = makeConnector();
    const f = celsiusFahrenheitConverterExpr(c);
    f.setValue(212, "user");
    expect(c.getValue()).toBe(100);
    expect(f.getValue()).toBe(212);
  });

  it("forgetting C clears the network, then F = 212 repropagates C", () => {
    const c = makeConnector();
    const f = celsiusFahrenheitConverterExpr(c);
    c.setValue(25, "user");
    expect(f.getValue()).toBe(77);
    c.forgetValue("user");
    expect(c.hasValue()).toBe(false);
    expect(f.hasValue()).toBe(false);
    f.setValue(212, "user");
    expect(c.getValue()).toBe(100);
    expect(f.getValue()).toBe(212);
  });
});
