// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeConnector } from "../../packages/ch3/src/03-mutable-data.js";

import { squarerFromMultiplier } from "./ex_3_34.js";

describe("exercise 3.34: Louis's squarer and its flaw", () => {
  it("setting a propagates: a = 7 makes b = 49", () => {
    const a = makeConnector();
    const b = makeConnector();
    squarerFromMultiplier(a, b);
    a.setValue(7, "user");
    expect(b.getValue()).toBe(49);
  });

  it("setting b determines nothing: a stays unset and nothing throws", () => {
    const a = makeConnector();
    const b = makeConnector();
    squarerFromMultiplier(a, b);
    expect(() => b.setValue(36, "user")).not.toThrow();
    expect(a.hasValue()).toBe(false);
    expect(b.getValue()).toBe(36);
  });
});
