// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

import type { Account } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.42: Ben Bitdiddle's pre-serialized account. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_42.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.42 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An account constructor, so one workload can run against every
 * account version. */
export type AccountMaker = (initialBalance: number) => Account;

/** Ben's `make-account`: the serialized withdraw and deposit are built
 * once, at account creation, and each request answers that very
 * procedure rather than a fresh wrapping of the message. */
export function makeAccountPreSerialized(_initialBalance: number): Account {
  throw new PendingSolution();
}

/** The text's dispatch-time version, under the same maker type. */
export function makeAccountDispatchSerialized(_initialBalance: number): Account {
  throw new PendingSolution();
}

/** A fixed concurrent workload: two withdrawals and two deposits
 * forked together on one account; answers the final balance. */
export function concurrentWorkloadFinal(_account: Account): Effect.Effect<number> {
  throw new PendingSolution();
}

/** Final balances of two concurrent 20 withdrawals, each serialized as
 * one atomic chunk, over both orders: the outcome set any correctly
 * serialized account version allows. */
export function serializedPairFinals(): ReadonlyArray<number> {
  throw new PendingSolution();
}

/** The same two withdrawals on raw two-step withdraws (access, then
 * set), no serialization: what serialization rules out. */
export function unserializedPairFinals(): ReadonlyArray<number> {
  throw new PendingSolution();
}
