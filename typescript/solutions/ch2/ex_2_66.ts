// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { none, type Option, some } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.66: `lookupTree` over a binary tree of records ordered by
 * the numerical keys. The shape mirrors the section's
 * `elementOfSetTree`: one key comparison per level, Theta(log n) for a
 * balanced tree.
 */

/** A record: a numerical key with its value, the section's tuple. */
export type RecordEntry<Value> = readonly [key: number, value: Value];

/** A binary tree of records ordered by key: the section's tree shape
 * carrying records at the nodes. */
export type RecordTree<Value> =
  | { readonly _tag: "Empty" }
  | {
      readonly _tag: "Node";
      readonly entry: RecordEntry<Value>;
      readonly left: RecordTree<Value>;
      readonly right: RecordTree<Value>;
    };

/** The empty record tree. */
export const emptyRecordTree: RecordTree<never> = { _tag: "Empty" };

/** Adjoins a record by key, mirroring `adjoinSetTree`. */
export const adjoinRecordTree = <Value>(
  record: RecordEntry<Value>,
  set: RecordTree<Value>,
): RecordTree<Value> => {
  if (set._tag === "Empty") {
    return { _tag: "Node", entry: record, left: set, right: set };
  }
  if (record[0] === set.entry[0]) {
    return set;
  }
  return record[0] < set.entry[0]
    ? { _tag: "Node", entry: set.entry, left: adjoinRecordTree(record, set.left), right: set.right }
    : {
        _tag: "Node",
        entry: set.entry,
        left: set.left,
        right: adjoinRecordTree(record, set.right),
      };
};

/** The record with the given key, or nothing: the book's `lookup`. */
export const lookupTree = <Value>(
  givenKey: number,
  set: RecordTree<Value>,
): Option<RecordEntry<Value>> => {
  if (set._tag === "Empty") {
    return none;
  }
  const [key, value] = set.entry;
  if (givenKey === key) {
    return some([key, value]);
  }
  return lookupTree(givenKey, givenKey < key ? set.left : set.right);
};
