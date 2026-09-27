// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.26: the (key, value) records organized as a binary tree
 * instead of an unordered chain, so lookup and insert descend one
 * `compare`-ordered path instead of scanning the records. A record
 * insert created on the way down keeps the tree's shape even before
 * it carries a value; `isValueEntry` says whether the record holds
 * the value stored under its key. The fields are mutable, the
 * section's spelling of the book's set-car!/set-cdr!: insert rewrites
 * the tree in place, never rebuilding it.
 */

/** One node of the binary-tree table: an ordered key, the value
 * stored under it when `isValueEntry`, and the two subtrees the
 * `compare` order splits keys into. */
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
 * in-order key listing that exhibits the ordering lookup exploits. */
export interface TreeTable<K> {
  lookup(key: K): number | undefined;
  insert(key: K, value: number): void;
  keysInOrder(): K[];
}

/** Builds an empty tree table ordered by `compare` (negative when
 * the first key sorts before the second). Insert descends from the
 * root and either replaces an existing key's value in place or conses
 * the new record onto the empty branch it falls off of; lookup
 * answers only where the descent lands on a value entry, so a key
 * whose record exists as spine alone answers nothing. */
export const makeTreeTable = <K>(compare: (a: K, b: K) => number): TreeTable<K> => {
  let root: TreeNode<K> | null = null;
  const lookup = (key: K): number | undefined => {
    let node = root;
    while (node !== null) {
      const order = compare(key, node.key);
      if (order === 0) {
        return node.isValueEntry ? node.value : undefined;
      }
      node = order < 0 ? node.left : node.right;
    }
    return undefined;
  };
  const insert = (key: K, value: number): void => {
    if (root === null) {
      root = { key, left: null, right: null, isValueEntry: true, value };
      return;
    }
    let node = root;
    for (;;) {
      const order = compare(key, node.key);
      if (order === 0) {
        node.isValueEntry = true;
        node.value = value;
        return;
      }
      if (order < 0) {
        if (node.left === null) {
          node.left = { key, left: null, right: null, isValueEntry: true, value };
          return;
        }
        node = node.left;
      } else {
        if (node.right === null) {
          node.right = { key, left: null, right: null, isValueEntry: true, value };
          return;
        }
        node = node.right;
      }
    }
  };
  const keysInOrder = (): K[] => {
    const keys: K[] = [];
    const walk = (node: TreeNode<K> | null): void => {
      if (node === null) {
        return;
      }
      walk(node.left);
      keys.push(node.key);
      walk(node.right);
    };
    walk(root);
    return keys;
  };
  return { lookup, insert, keysInOrder };
};
