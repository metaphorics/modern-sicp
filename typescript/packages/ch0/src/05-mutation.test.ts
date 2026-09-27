// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: Chapter 0 section 0.5

import { describe, expect, it } from "vitest";

import { bumpedThroughAlias, type Cell, makeCounter, makeWithdrawer } from "./05-mutation.js";

describe("mutation and ownership", () => {
  it("two calls make two cells", () => {
    const c1 = makeCounter(0);
    const c2 = makeCounter(0);
    expect(c1()).toBe(1);
    expect(c1()).toBe(2);
    expect(c2()).toBe(1);
  });

  it("the withdrawer shares one balance across calls", () => {
    const w = makeWithdrawer(100);
    expect(w(30)).toBe(70);
    expect(w(20)).toBe(50);
  });

  it("mutation is visible through every name that holds the object", () => {
    expect(bumpedThroughAlias()).toBe(9);
  });

  it("identity is what === asks about objects", () => {
    const cell: Cell = { value: 1 };
    const alias = cell;
    const stranger: Cell = { value: 1 };
    expect(alias === cell).toBe(true);
    expect(alias).toEqual({ value: 1 });
    expect(stranger === cell).toBe(false);
  });
});
