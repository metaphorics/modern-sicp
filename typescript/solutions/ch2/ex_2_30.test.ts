// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { leaf, node, showTree } from "../../packages/ch2/src/02-picture-language.js";
import { squareTreeDirect, squareTreeViaMap } from "./ex_2_30.js";

const tree = node(leaf(1), node(leaf(2), node(leaf(3), leaf(4)), leaf(5)), node(leaf(6), leaf(7)));

describe("exercise 2.30", () => {
  it("direct recursion squares the leaves in place", () => {
    expect(showTree(squareTreeDirect(tree))).toBe("[1, [4, [9, 16], 25], [36, 49]]");
  });

  it("the map spelling squares the same tree identically", () => {
    expect(showTree(squareTreeViaMap(tree))).toBe("[1, [4, [9, 16], 25], [36, 49]]");
  });
});
