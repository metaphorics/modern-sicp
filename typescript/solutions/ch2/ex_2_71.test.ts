// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import type { HuffTree } from "../../packages/ch2/src/03-symbolic-data.js";
import { geometricTree, leastFrequentBits, mostFrequentBits } from "./ex_2_71.js";

/** Walks the left spine, counting its length: the skewed chain check. */
const leftSpine = (tree: HuffTree): number => {
  let depth = 0;
  for (let node: HuffTree = tree; node._tag === "Branch"; node = node.left) {
    depth += 1;
  }
  return depth;
};

const rightmostLeafWeight = (tree: HuffTree): number =>
  tree._tag === "Leaf" ? tree.weight : rightmostLeafWeight(tree.right);

describe("exercise 2.71", () => {
  it("n = 5: the most frequent symbol needs 1 bit, the least needs 4", () => {
    expect(mostFrequentBits(5)).toBe(1);
    expect(leastFrequentBits(5)).toBe(4);
  });

  it("n = 10: 1 bit and 9 bits", () => {
    expect(mostFrequentBits(10)).toBe(1);
    expect(leastFrequentBits(10)).toBe(9);
  });

  it("the tree is the maximally skewed chain, powers along the spine", () => {
    const tree = geometricTree(5);
    expect(tree._tag).toBe("Ok");
    if (tree._tag !== "Ok") {
      return;
    }
    // The left spine runs the whole height; the powers of two peel off
    // to the right one merge at a time.
    expect(leftSpine(tree.value)).toBe(4);
    expect(rightmostLeafWeight(tree.value)).toBe(16);
  });
});
