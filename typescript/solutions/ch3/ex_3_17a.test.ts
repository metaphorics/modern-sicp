// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { type MList, mlist, mnil } from "../../packages/ch3/src/03-mutable-data.js";
import { makeZ1, makeZ2 } from "./ex_3_15.js";
import { countNodes } from "./ex_3_17a.js";

describe("exercise 3.17a: counting distinct nodes", () => {
  it("the shared z1 has 3 pairs and 2 atoms: 5 nodes", () => {
    expect(countNodes(makeZ1())).toBe(5);
  });

  it("the unshared z2 has 5 pairs and 2 atoms: 7 nodes", () => {
    expect(countNodes(makeZ2())).toBe(7);
  });

  it("the unshared tree ((a b) c d) has 5 pairs and 4 atoms: 9 nodes", () => {
    expect(countNodes(mlist<MList<string> | string>(mlist("a", "b"), "c", "d"))).toBe(9);
  });

  it("equal atom values count once", () => {
    expect(countNodes(mlist("a", "a", "b"))).toBe(5);
  });

  it("the empty list has no nodes", () => {
    expect(countNodes(mnil)).toBe(0);
  });
});
