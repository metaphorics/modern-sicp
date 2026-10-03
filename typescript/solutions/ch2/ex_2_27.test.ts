// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { leaf, node, showTree } from "../../packages/ch2/src/02-picture-language.js";

import { deepReverse, shallowReverse } from "./ex_2_27.js";

describe("exercise 2.27", () => {
  const x = node(node(leaf(1), leaf(2)), node(leaf(3), leaf(4)));

  it("the tree prints as the book's x", () => {
    expect(showTree(x)).toBe("[[1, 2], [3, 4]]");
  });

  it("shallow reversal flips only the outermost order", () => {
    expect(showTree(shallowReverse(x))).toBe("[[3, 4], [1, 2]]");
  });

  it("deep reversal flips every level", () => {
    expect(showTree(deepReverse(x))).toBe("[[4, 3], [2, 1]]");
  });

  it("a deeper tree reverses all the way down", () => {
    expect(showTree(deepReverse(node(leaf(1), node(leaf(2), leaf(3)))))).toBe("[[3, 2], 1]");
  });
});
