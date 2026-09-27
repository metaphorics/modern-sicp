// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { ContradictionError, makeConnector } from "../../packages/ch3/src/03-mutable-data.js";

import { averager } from "./ex_3_33.js";

describe("exercise 3.33: averager", () => {
  it("a = 10 and b = 20 make c = 15", () => {
    const a = makeConnector();
    const b = makeConnector();
    const c = makeConnector();
    averager(a, b, c);
    a.setValue(10, "user");
    b.setValue(20, "user");
    expect(c.getValue()).toBe(15);
  });

  it("forgetting withdraws the averager's claims, then a = 10 and c = 25 make b = 40", () => {
    const a = makeConnector();
    const b = makeConnector();
    const c = makeConnector();
    averager(a, b, c);
    a.setValue(10, "user");
    b.setValue(20, "user");
    expect(c.getValue()).toBe(15);
    a.forgetValue("user");
    expect(a.hasValue()).toBe(false);
    expect(b.getValue()).toBe(20);
    expect(c.hasValue()).toBe(false);
    b.forgetValue("user");
    expect(b.hasValue()).toBe(false);
    a.setValue(10, "user");
    c.setValue(25, "user");
    expect(b.getValue()).toBe(40);
  });

  it("b = 40 and c = 25 make a = 10", () => {
    const a = makeConnector();
    const b = makeConnector();
    const c = makeConnector();
    averager(a, b, c);
    b.setValue(40, "user");
    c.setValue(25, "user");
    expect(a.getValue()).toBe(10);
  });

  it("setting c to a value inconsistent with the average is a contradiction", () => {
    const a = makeConnector();
    const b = makeConnector();
    const c = makeConnector();
    averager(a, b, c);
    a.setValue(10, "user");
    b.setValue(20, "user");
    expect(c.getValue()).toBe(15);
    expect(() => c.setValue(20, "user")).toThrow(ContradictionError);
  });
});
