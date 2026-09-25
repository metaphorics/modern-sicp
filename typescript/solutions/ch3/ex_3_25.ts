// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { isMNil, type MList, mcons, mnil } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.25: generalizing one- and two-dimensional tables, a
 * table in which values are stored under key lists of arbitrary
 * length, the book's `lookup`/`insert!` taking the key list as
 * input. The section's tables hang a chain of records off the table
 * object; the generalization hangs records off records: each record
 * carries the value stored under the full key path ending at it and
 * the subtable of records the next key chooses among, so one record
 * can be spine and value at once (`["a", "b", "c"]` and
 * `["a", "b", "c", "e"]` share their spine). The table object is the
 * sentinel record the book prints as `*table*`; insert mutates the
 * chains in place, the section's `insert!` discipline.
 */

/** One record of the nested table: a key, the value stored under the
 * key path ending here (`undefined` while the record is spine only),
 * and the subtable chain the next key descends into. */
export interface NestedNode {
  readonly key: string;
  value: number | undefined;
  records: MList<NestedNode>;
}

/** The book's generalized table object: the sentinel record the
 * top-level keys hang from, answering the two operations as
 * messages. */
export interface NestedTable {
  lookup(keys: string[]): number | undefined;
  insert(keys: string[], value: number): void;
}

/** Scans a records chain for `key`, the book's `assoc` over the
 * section's pairs; answers nothing when the key is absent. */
const findNode = (records: MList<NestedNode>, key: string): NestedNode | undefined => {
  for (let rest = records; !isMNil(rest); rest = rest.tail) {
    if (rest.head.key === key) {
      return rest.head;
    }
  }
  return undefined;
};

/** Builds the empty nested table. `lookup` walks the spine and
 * answers the value only where the full key path ends on a record
 * that carries one, so an intermediate spine record is no value;
 * `insert` walks the spine consing missing records onto their
 * parent's chain and sets the final key's value, replacing it in
 * place when the record is already there. */
export const makeNestedTable = (): NestedTable => {
  const table: NestedNode = { key: "*table*", value: undefined, records: mnil };
  const lookup = (keys: string[]): number | undefined => {
    let node = table;
    for (const key of keys) {
      const next = findNode(node.records, key);
      if (next === undefined) {
        return undefined;
      }
      node = next;
    }
    return node.value;
  };
  const insert = (keys: string[], value: number): void => {
    const lastKey = keys.at(-1);
    if (lastKey === undefined) {
      throw new Error("insert: needs at least one key");
    }
    let node = table;
    for (const key of keys.slice(0, -1)) {
      let next = findNode(node.records, key);
      if (next === undefined) {
        next = { key, value: undefined, records: mnil };
        node.records = mcons(next, node.records);
      }
      node = next;
    }
    const leaf = findNode(node.records, lastKey);
    if (leaf === undefined) {
      node.records = mcons({ key: lastKey, value, records: mnil }, node.records);
    } else {
      leaf.value = value;
    }
  };
  return { lookup, insert };
};
