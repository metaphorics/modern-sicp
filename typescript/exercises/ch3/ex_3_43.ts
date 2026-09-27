// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.43: exchange preserves the multiset of balances. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_43.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.43 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The three account balances, a1 first. */
export type Balances = readonly [number, number, number];

/** The first version of `exchange` over individually serialized
 * accounts: read both balances, then withdraw the difference from one
 * account and deposit it into the other, each update an atomic chunk. */
export function firstVersionFinals(): ReadonlyArray<Balances> {
  throw new PendingSolution();
}

/** The first distinct first-version final whose multiset is not {10,
 * 20, 30}, or undefined when every schedule preserved it. */
export function firstMultisetViolatingFinal(): Balances | undefined {
  throw new PendingSolution();
}

/** The distinct sums the first-version schedules produce: only 60,
 * since each exchange moves a matched amount. */
export function firstVersionSums(): ReadonlyArray<number> {
  throw new PendingSolution();
}

/** The serialized exchange: the whole swap is one chunk per process,
 * so the two exchanges run in one of two orders and the balances stay
 * a permutation of the start. */
export function serializedFinals(): ReadonlyArray<Balances> {
  throw new PendingSolution();
}

/** The first version again, with the per-account serialization
 * removed: each withdraw and deposit is two raw steps (access, then
 * set), so two processes can interleave inside a single update and
 * lose money outright. */
export function noAccountSerializationFinals(): ReadonlyArray<Balances> {
  throw new PendingSolution();
}

/** The distinct sums the unserialized schedules produce, ascending:
 * every sum below 60 is money the races destroyed. */
export function noAccountSerializationSums(): ReadonlyArray<number> {
  throw new PendingSolution();
}
