// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Table } from "../../packages/ch3/src/03-mutable-data.js";
import { makeTable, tableInsert, tableLookup } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.24: the book's `make-table` takes a `same-key?` predicate
 * and returns a `lookup` and an `insert!` pair that share one local
 * table. The edition spells the message-passing constructor over the
 * section's table: the local table is a module `Table`, the two
 * closures pass the predicate through to the section's lookup and
 * insert, and replacing an existing key's value in place is the
 * predicate's job to detect.
 */

/** The book's table interface for 3.24: lookup answers the stored
 * value or undefined (the book's false), insert associates the key
 * with the value, replacing in place when `sameKey?` holds. */
export interface SameKeyTable<K, V> {
  readonly lookup: (key: K) => V | undefined;
  readonly insert: (key: K, value: V) => void;
}

/** Builds a table whose key comparisons go through `sameKey?`: the
 * returned closures share the one local table, so inserts are visible
 * to later lookups and replacing a matched key's value is an in-place
 * record update, exactly the book's dispatch closures. */
export const makeTableSameKey = <K, V>(sameKey: (a: K, b: K) => boolean): SameKeyTable<K, V> => {
  const localTable: Table<K, V> = makeTable();
  return {
    lookup: (key) => tableLookup(localTable, key, sameKey),
    insert: (key, value) => {
      tableInsert(localTable, key, value, sameKey);
    },
  };
};
