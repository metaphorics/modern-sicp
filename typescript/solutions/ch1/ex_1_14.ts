// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.14: the tree the count-change process generates, measured.
 *
 * Every call this local cc makes is one node of the tree the exercise
 * asks to draw, and the deepest chain of simultaneous calls is the
 * space the process needs. The drawing and the growth reading live in
 * ex_1_14.md; this file is the measurement.
 */
const firstDenomination = (kindsOfCoins: number): number => {
  if (kindsOfCoins === 1) {
    return 1;
  }
  if (kindsOfCoins === 2) {
    return 5;
  }
  if (kindsOfCoins === 3) {
    return 10;
  }
  if (kindsOfCoins === 4) {
    return 25;
  }
  return 50;
};

export function countChangeWithNodes(amount: number): {
  ways: number;
  nodes: number;
  maxDepth: number;
} {
  let nodes = 0;
  let depth = 0;
  let maxDepth = 0;
  const cc = (a: number, kinds: number): number => {
    nodes += 1;
    depth += 1;
    if (depth > maxDepth) {
      maxDepth = depth;
    }
    if (a === 0) {
      depth -= 1;
      return 1;
    }
    if (a < 0 || kinds === 0) {
      depth -= 1;
      return 0;
    }
    const ways = cc(a, kinds - 1) + cc(a - firstDenomination(kinds), kinds);
    depth -= 1;
    return ways;
  };
  const ways = cc(amount, 5);
  return { ways, nodes, maxDepth };
}
