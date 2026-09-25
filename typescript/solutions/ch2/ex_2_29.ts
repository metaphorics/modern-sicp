// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.29: binary mobiles as a tagged union, their total weight,
 * and their balance. Part d rebuilds the representation as pairs — the
 * mobile a pair of branches, the branch a length/structure pair — where
 * only the constructors and selectors change.
 */

/** A mobile hangs two branches; a weight is a plain load. */
export type Weight = { readonly _tag: "Weight"; readonly weight: number };
export type Mobile = { readonly _tag: "Mobile"; readonly left: Branch; readonly right: Branch };
export type Structure = Mobile | Weight;

/** A branch holds its length and the structure at its end. */
export type Branch = {
  readonly _tag: "Branch";
  readonly length: number;
  readonly structure: Structure;
};

/** Builds a mobile from its two branches. */
export function makeMobile(left: Branch, right: Branch): Structure {
  return { _tag: "Mobile", left, right };
}

/** Builds a branch of the given length carrying `structure`. */
export function makeBranch(length: number, structure: Structure): Branch {
  return { _tag: "Branch", length, structure };
}

/** Builds a plain load. */
export function makeWeight(weight: number): Structure {
  return { _tag: "Weight", weight };
}

/** The mobile's left branch. */
export function leftBranch(mobile: Mobile): Branch {
  return mobile.left;
}

/** The mobile's right branch. */
export function rightBranch(mobile: Mobile): Branch {
  return mobile.right;
}

/** The branch's length. */
export function branchLength(branch: Branch): number {
  return branch.length;
}

/** The structure the branch carries. */
export function branchStructure(branch: Branch): Structure {
  return branch.structure;
}

/** The total weight the structure carries. */
export function totalWeight(structure: Structure): number {
  if (structure._tag === "Weight") {
    return structure.weight;
  }
  return (
    totalWeight(branchStructure(leftBranch(structure))) +
    totalWeight(branchStructure(rightBranch(structure)))
  );
}

/** A branch's torque: its length times the weight it carries. */
const torque = (branch: Branch): number =>
  branchLength(branch) * totalWeight(branchStructure(branch));

/** Whether every torque in the structure evens out, submobiles included. */
export function isBalanced(structure: Structure): boolean {
  if (structure._tag === "Weight") {
    return true;
  }
  const left = leftBranch(structure);
  const right = rightBranch(structure);
  return (
    torque(left) === torque(right) &&
    isBalanced(branchStructure(left)) &&
    isBalanced(branchStructure(right))
  );
}

// Part d: the same mobiles as pairs. Only the constructors and the
// selectors change; the weight and balance logic is applied unchanged
// through the re-tagged view below.

/** Part d: builds the mobile as a pair of branches. */
export function makeMobilePair(left: Branch, right: Branch): readonly [Branch, Branch] {
  return [left, right];
}

/** Part d: builds a branch as a length/structure pair. */
export function makeBranchPair(length: number, structure: Structure): readonly [number, Structure] {
  return [length, structure];
}

/** Part d: the mobile's left branch. */
export function leftBranchPair(mobile: readonly [Branch, Branch]): Branch {
  return mobile[0];
}

/** Part d: the mobile's right branch. */
export function rightBranchPair(mobile: readonly [Branch, Branch]): Branch {
  return mobile[1];
}

/** Part d: the branch's length. */
export function branchLengthPair(branch: readonly [number, Structure]): number {
  return branch[0];
}

/** Part d: the structure the branch carries. */
export function branchStructurePair(branch: readonly [number, Structure]): Structure {
  return branch[1];
}

/** Part d: total weight through the pair selectors. */
export function totalWeightPair(mobile: readonly [Branch, Branch]): number {
  return totalWeight(makeMobile(leftBranchPair(mobile), rightBranchPair(mobile)));
}

/** Part d: balance through the pair selectors. */
export function isBalancedPair(mobile: readonly [Branch, Branch]): boolean {
  return isBalanced(makeMobile(leftBranchPair(mobile), rightBranchPair(mobile)));
}
