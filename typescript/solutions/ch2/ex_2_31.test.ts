// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { leaf, node, showTree } from "../../packages/ch2/src/02-picture-language.js";
import { squareTree, treeMap } from "./ex_2_31.js";

const tree = node(leaf(1), node(leaf(2), node(leaf(3), leaf(4)), leaf(5)), node(leaf(6), leaf(7)));

describe("exercise 2.31", () => {
  it("treeMap squares the leaves of the 2.30 tree", () => {
    expect(showTree(treeMap((x) => x * x, tree))).toBe("[1, [4, [9, 16], 25], [36, 49]]");
  });

  it("treeMap takes any leaf function", () => {
    expect(showTree(treeMap((x) => x + 1, node(leaf(1), leaf(2))))).toBe("[2, 3]");
  });

  it("square-tree is treeMap with the squaring function", () => {
    expect(showTree(squareTree(tree))).toBe("[1, [4, [9, 16], 25], [36, 49]]");
  });
});
