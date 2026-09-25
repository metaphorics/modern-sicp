// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeTableSameKey } from "./ex_3_24.js";

const sameKeyCaseInsensitive = (a: string, b: string): boolean =>
  a.toLowerCase() === b.toLowerCase();

describe("exercise 3.24: the table keyed by same-key?", () => {
  it("compares keys through the supplied predicate", () => {
    const table = makeTableSameKey(sameKeyCaseInsensitive);
    expect(table.lookup("nonexistent")).toBeUndefined();
    table.insert("a", 1);
    expect(table.lookup("a")).toBe(1);
    // The predicate, not the stored spelling, finds the record.
    expect(table.lookup("A")).toBe(1);
    table.insert("b", 2);
    expect(table.lookup("B")).toBe(2);
    expect(table.lookup("c")).toBeUndefined();
  });

  it("replaces the value when same-key? matches an existing record", () => {
    const table = makeTableSameKey(sameKeyCaseInsensitive);
    table.insert("Warp", 10);
    table.insert("warp", 25);
    // One record, updated in place: the new value wins under every
    // spelling the predicate matches.
    expect(table.lookup("warp")).toBe(25);
    expect(table.lookup("WARP")).toBe(25);
  });

  it("distinct predicates build distinct tables", () => {
    const loose = makeTableSameKey(sameKeyCaseInsensitive);
    const strict = makeTableSameKey<string, number>((a, b) => a === b);
    loose.insert("a", 1);
    strict.insert("a", 2);
    expect(loose.lookup("A")).toBe(1);
    expect(strict.lookup("A")).toBeUndefined();
    expect(strict.lookup("a")).toBe(2);
  });
});
