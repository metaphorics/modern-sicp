// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.24: the table keyed by same-key?. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_24.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.24 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's table interface: lookup answers the value or
 * undefined, insert replaces when sameKey? matches. */
export interface SameKeyTable<K, V> {
  readonly lookup: (key: K) => V | undefined;
  readonly insert: (key: K, value: V) => void;
}

/** Builds a table whose key comparisons go through `sameKey?`. */
export function makeTableSameKey<K, V>(_sameKey: (a: K, b: K) => boolean): SameKeyTable<K, V> {
  throw new PendingSolution();
}
