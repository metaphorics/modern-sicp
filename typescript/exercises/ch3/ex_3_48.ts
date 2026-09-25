// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

import type { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";
import type { AccountWithSerializer, Mutex } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.48: deadlock avoidance by lock ordering. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_48.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.48 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An account carrying its unique number and the mutex its serializer
 * wraps. The mutex travels with the object because the race
 * demonstration needs the two acquire points the serializer otherwise
 * hides. */
export interface NumberedAccount extends AccountWithSerializer {
  readonly accountNumber: number;
  readonly balanceMutex: Mutex;
}

/** The book's modified `make-account`: every account is created with a
 * unique number, here drawn from a process-wide counter, and its
 * serializer wraps a mutex the account can name. */
export function makeNumberedAccount(_initialBalance: number): NumberedAccount {
  throw new PendingSolution();
}

/** The book's rewritten `serialized-exchange`: the account with the
 * smaller number is always serialized outermost, so every process
 * enters its serializers in the same ascending order. */
export function serializedExchangeOrdered(
  _account1: NumberedAccount,
  _account2: NumberedAccount,
): Effect.Effect<void, InsufficientFunds> {
  throw new PendingSolution();
}

/** Runs `effect` on its own fiber under a real-time limit in
 * milliseconds and answers whether the limit fired. */
export function timesOut<A, E>(
  _effect: Effect.Effect<A, E>,
  _millis: number,
): Effect.Effect<boolean> {
  throw new PendingSolution();
}

/** Peter exchanges a1 with a2 while Paul exchanges a2 with a1, both
 * taking the lower number first. Answers whether the race completes:
 * always true, since a wait cycle would need a descending edge. */
export function orderedRaceCompletes(
  _account1: NumberedAccount,
  _account2: NumberedAccount,
): Effect.Effect<boolean> {
  throw new PendingSolution();
}

/** The same opposing exchanges in the text's nesting order, each
 * process acquiring its own account first. Answers whether the race
 * deadlocks: true, each fiber ends up holding one mutex and waiting
 * forever for the other. */
export function unorderedRaceDeadlocks(
  _account1: NumberedAccount,
  _account2: NumberedAccount,
): Effect.Effect<boolean> {
  throw new PendingSolution();
}
