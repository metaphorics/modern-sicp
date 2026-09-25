// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { ContradictionError, makeConnector } from "../../packages/ch3/src/03-mutable-data.js";

import { squarer } from "./ex_3_35.js";

describe("exercise 3.35: squarer as a primitive constraint", () => {
  it("a = 7 makes b = 49", () => {
    const a = makeConnector();
    const b = makeConnector();
    squarer(a, b);
    a.setValue(7, "user");
    expect(b.getValue()).toBe(49);
  });

  it("b = 49 makes a = 7", () => {
    const a = makeConnector();
    const b = makeConnector();
    squarer(a, b);
    b.setValue(49, "user");
    expect(a.getValue()).toBe(7);
  });

  it("forgetting withdraws both claims, then b = 49 repropagates a", () => {
    const a = makeConnector();
    const b = makeConnector();
    squarer(a, b);
    a.setValue(7, "user");
    expect(b.getValue()).toBe(49);
    a.forgetValue("user");
    expect(a.hasValue()).toBe(false);
    expect(b.hasValue()).toBe(false);
    b.setValue(49, "user");
    expect(a.getValue()).toBe(7);
  });

  it("setting b inconsistent with a is a contradiction", () => {
    const a = makeConnector();
    const b = makeConnector();
    squarer(a, b);
    a.setValue(7, "user");
    expect(b.getValue()).toBe(49);
    expect(() => b.setValue(50, "user")).toThrow(ContradictionError);
  });

  it("a negative b is refused: a square less than 0", () => {
    const a = makeConnector();
    const b = makeConnector();
    squarer(a, b);
    expect(() => b.setValue(-4, "user")).toThrow("squarer: negative");
    expect(a.hasValue()).toBe(false);
  });
});
