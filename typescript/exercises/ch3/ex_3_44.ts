// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

import type { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";
import type { Account } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.44: transfer needs no joint lock. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_44.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.44 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `transfer`: withdraw from one account, deposit into the
 * other, with no joint serialization beyond what each account already
 * applies to its own operations. */
export function transfer(
  _fromAccount: Account,
  _toAccount: Account,
  _amount: number,
): Effect.Effect<void, InsufficientFunds> {
  throw new PendingSolution();
}

/** Runs the six-transfer workload on serialized accounts and answers
 * the total afterwards: 300, whatever the interleaving. */
export function totalAfterConcurrentTransfers(): Effect.Effect<number> {
  throw new PendingSolution();
}

/** The same workload on raw unserialized accounts: the contrast total,
 * below or above 300 when the races eat a deposit or revive a
 * withdrawal. */
export function totalAfterUnserializedTransfers(): Effect.Effect<number> {
  throw new PendingSolution();
}
