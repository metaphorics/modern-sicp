// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeNestedTable } from "./ex_3_25.js";

describe("exercise 3.25: tables under arbitrary key lists", () => {
  it("stores and finds values under two- and three-key paths", () => {
    const table = makeNestedTable();
    table.insert(["a", "b", "c"], 1);
    table.insert(["a", "b", "d"], 2);
    table.insert(["a", "x"], 3);
    expect(table.lookup(["a", "b", "c"])).toBe(1);
    expect(table.lookup(["a", "b", "d"])).toBe(2);
    expect(table.lookup(["a", "x"])).toBe(3);
  });

  it("intermediate spine records are not values", () => {
    const table = makeNestedTable();
    table.insert(["a", "b", "c"], 1);
    table.insert(["a", "b", "d"], 2);
    table.insert(["a", "x"], 3);
    expect(table.lookup(["a", "b"])).toBeUndefined();
    expect(table.lookup(["a"])).toBeUndefined();
  });

  it("a missing path answers undefined", () => {
    const table = makeNestedTable();
    table.insert(["a", "b", "c"], 1);
    expect(table.lookup(["q", "r"])).toBeUndefined();
  });

  it("inserting along an existing spine extends it", () => {
    const table = makeNestedTable();
    table.insert(["a", "b", "c"], 1);
    table.insert(["a", "b", "d"], 2);
    table.insert(["a", "x"], 3);
    table.insert(["a", "b", "c", "e"], 4);
    expect(table.lookup(["a", "b", "c", "e"])).toBe(4);
    expect(table.lookup(["a", "b", "c"])).toBe(1);
    expect(table.lookup(["a", "b", "d"])).toBe(2);
    expect(table.lookup(["a", "x"])).toBe(3);
  });
});
