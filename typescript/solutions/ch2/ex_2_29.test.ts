// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  branchLengthPair,
  branchStructurePair,
  isBalanced,
  isBalancedPair,
  leftBranchPair,
  makeBranch,
  makeBranchPair,
  makeMobile,
  makeMobilePair,
  makeWeight,
  rightBranchPair,
  totalWeight,
  totalWeightPair,
} from "./ex_2_29.js";

describe("exercise 2.29", () => {
  const left = makeBranch(2, makeWeight(5));
  const right = makeBranch(5, makeWeight(2));
  const simple = makeMobile(left, right);

  it("the simple mobile weighs 7 and balances", () => {
    expect(totalWeight(simple)).toBe(7);
    expect(isBalanced(simple)).toBe(true);
  });

  it("the nested mobile weighs 10 but its two sides do not even out", () => {
    const sub = makeMobile(makeBranch(3, makeWeight(4)), makeBranch(4, makeWeight(3)));
    const nested = makeMobile(makeBranch(2, makeWeight(3)), makeBranch(2, sub));
    expect(totalWeight(nested)).toBe(10);
    expect(isBalanced(nested)).toBe(false);
    expect(isBalanced(sub)).toBe(true);
  });

  it("part d: the pair selectors return the same parts", () => {
    const pair = makeMobilePair(left, right);
    expect(leftBranchPair(pair)).toBe(left);
    expect(rightBranchPair(pair)).toBe(right);
    expect(totalWeightPair(pair)).toBe(7);
    expect(isBalancedPair(pair)).toBe(true);
    const branch = makeBranchPair(2, makeWeight(5));
    expect(branchLengthPair(branch)).toBe(2);
    expect(branchStructurePair(branch)).toStrictEqual(makeWeight(5));
  });
});
