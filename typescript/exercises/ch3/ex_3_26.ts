// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.26: the (key, value) records of a table organized as a
 * binary tree. Pending scaffold; the solution and its rationale live
 * in solutions/ch3/ex_3_26.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.26 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One node of the binary-tree table: an ordered key, the value
 * stored under it when `isValueEntry`, and the two subtrees the
 * `compare` order splits keys into. The fields are mutable: insert
 * rewrites them in place. */
export interface TreeEntry<K> {
  key: K;
  left: TreeNode<K> | null;
  right: TreeNode<K> | null;
  isValueEntry: boolean;
  value: number;
}

/** A subtree: an entry, or the empty branch the book's `'()` spells
 * as `null`. */
export type TreeNode<K> = TreeEntry<K>;

/** The book's binary-tree table object: the two operations plus the
 * in-order key listing. */
export interface TreeTable<K> {
  lookup(key: K): number | undefined;
  insert(key: K, value: number): void;
  keysInOrder(): K[];
}

/** Builds an empty tree table ordered by `compare` (negative when
 * the first key sorts before the second). */
export function makeTreeTable<K>(_compare: (a: K, b: K) => number): TreeTable<K> {
  throw new PendingSolution();
}
