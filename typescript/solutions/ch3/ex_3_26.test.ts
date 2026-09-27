// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeTreeTable } from "./ex_3_26.js";

const numberCompare = (a: number, b: number): number => a - b;

describe("exercise 3.26: a binary-tree table", () => {
  it("lists keys in order after out-of-order inserts", () => {
    const table = makeTreeTable(numberCompare);
    table.insert(5, 50);
    table.insert(2, 20);
    table.insert(8, 80);
    table.insert(1, 10);
    table.insert(3, 30);
    expect(table.keysInOrder()).toEqual([1, 2, 3, 5, 8]);
  });

  it("finds present keys and misses absent ones", () => {
    const table = makeTreeTable(numberCompare);
    table.insert(5, 50);
    table.insert(2, 20);
    table.insert(8, 80);
    table.insert(1, 10);
    table.insert(3, 30);
    expect(table.lookup(3)).toBe(30);
    expect(table.lookup(5)).toBe(50);
    expect(table.lookup(1)).toBe(10);
    expect(table.lookup(4)).toBeUndefined();
    expect(table.lookup(9)).toBeUndefined();
  });

  it("replacing a key's value rewrites the record in place", () => {
    const table = makeTreeTable(numberCompare);
    table.insert(5, 50);
    table.insert(2, 20);
    table.insert(8, 80);
    table.insert(1, 10);
    table.insert(3, 30);
    table.insert(2, 99);
    expect(table.lookup(2)).toBe(99);
    expect(table.keysInOrder()).toEqual([1, 2, 3, 5, 8]);
  });

  it("a spine-only path answers undefined", () => {
    const table = makeTreeTable(numberCompare);
    table.insert(5, 50);
    expect(table.lookup(3)).toBeUndefined();
    expect(table.keysInOrder()).toEqual([5]);
  });

  it("works for any ordered key type", () => {
    const table = makeTreeTable((a: string, b: string) => (a < b ? -1 : a > b ? 1 : 0));
    table.insert("pear", 1);
    table.insert("apple", 2);
    table.insert("fig", 3);
    expect(table.keysInOrder()).toEqual(["apple", "fig", "pear"]);
    expect(table.lookup("fig")).toBe(3);
  });
});
