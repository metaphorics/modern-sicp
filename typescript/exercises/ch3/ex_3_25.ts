// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.25: a table whose values sit under key lists of
 * arbitrary length. Pending scaffold; the solution and its rationale
 * live in solutions/ch3/ex_3_25.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.25 is not solved yet");
    this.name = "PendingSolution";
  }
}

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

/** Builds the empty nested table. */
export function makeNestedTable(): NestedTable {
  throw new PendingSolution();
}
