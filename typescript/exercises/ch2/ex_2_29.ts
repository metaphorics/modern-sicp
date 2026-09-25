// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 2.29: mobiles, their weight, and their balance. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.29 is not solved yet");
    this.name = "PendingSolution";
  }
}

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
export function makeMobile(_left: Branch, _right: Branch): Structure {
  throw new PendingSolution();
}

/** Builds a branch of the given length carrying `structure`. */
export function makeBranch(_length: number, _structure: Structure): Branch {
  throw new PendingSolution();
}

/** Builds a plain load. */
export function makeWeight(_weight: number): Structure {
  throw new PendingSolution();
}

/** The mobile's left branch. */
export function leftBranch(_mobile: Mobile): Branch {
  throw new PendingSolution();
}

/** The mobile's right branch. */
export function rightBranch(_mobile: Mobile): Branch {
  throw new PendingSolution();
}

/** The branch's length. */
export function branchLength(_branch: Branch): number {
  throw new PendingSolution();
}

/** The structure the branch carries. */
export function branchStructure(_branch: Branch): Structure {
  throw new PendingSolution();
}

/** The total weight the structure carries. */
export function totalWeight(_structure: Structure): number {
  throw new PendingSolution();
}

/** Whether every torque in the structure evens out, submobiles included. */
export function isBalanced(_structure: Structure): boolean {
  throw new PendingSolution();
}

/** Part d: builds the mobile as a pair of branches. */
export function makeMobilePair(_left: Branch, _right: Branch): readonly [Branch, Branch] {
  throw new PendingSolution();
}

/** Part d: builds a branch as a length/structure pair. */
export function makeBranchPair(
  _length: number,
  _structure: Structure,
): readonly [number, Structure] {
  throw new PendingSolution();
}

/** Part d: the mobile's left branch. */
export function leftBranchPair(_mobile: readonly [Branch, Branch]): Branch {
  throw new PendingSolution();
}

/** Part d: the mobile's right branch. */
export function rightBranchPair(_mobile: readonly [Branch, Branch]): Branch {
  throw new PendingSolution();
}

/** Part d: the branch's length. */
export function branchLengthPair(_branch: readonly [number, Structure]): number {
  throw new PendingSolution();
}

/** Part d: the structure the branch carries. */
export function branchStructurePair(_branch: readonly [number, Structure]): Structure {
  throw new PendingSolution();
}
