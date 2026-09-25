// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

import type { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";
import type { AccountWithSerializer } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.45: double serialization deadlocks. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_45.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.45 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Louis's `make-account-and-serializer`: withdraw and deposit are
 * protected, and the serializer message answers that same protector. */
export function makeLouisAccount(_initialBalance: number): AccountWithSerializer {
  throw new PendingSolution();
}

/** Louis's deposit usage: the account's deposit is already serialized,
 * so the caller just applies it. */
export function louisDeposit(
  _account: AccountWithSerializer,
  _amount: number,
): Effect.Effect<number, InsufficientFunds> {
  throw new PendingSolution();
}

/** Runs `effect` on its own fiber under a real-time limit in
 * milliseconds and answers whether the limit fired: true means the
 * fiber never finished, the signature of a deadlock. */
export function timesOut<A, E>(
  _effect: Effect.Effect<A, E>,
  _millis: number,
): Effect.Effect<boolean> {
  throw new PendingSolution();
}

/** Whether a serialized exchange of the two given accounts deadlocks
 * when both are Louis accounts: always true, whichever order. */
export function louisExchangeDeadlocks(
  _account1: AccountWithSerializer,
  _account2: AccountWithSerializer,
): Effect.Effect<boolean> {
  throw new PendingSolution();
}

/** Whether the module's explicit-serializer deposit, which wraps the
 * account's deposit in the exported serializer once more, deadlocks on
 * a Louis account: true, two acquisitions of one non-reentrant
 * serializer on one path. */
export function moduleDepositOnLouisDeadlocks(
  _account: AccountWithSerializer,
): Effect.Effect<boolean> {
  throw new PendingSolution();
}

/** The control: a serialized exchange of plain accounts, whose
 * withdraw and deposit are raw, completes. */
export function plainExchangeCompletes(
  _account1: AccountWithSerializer,
  _account2: AccountWithSerializer,
): Effect.Effect<boolean> {
  throw new PendingSolution();
}

/** A plain account factory for the controls. */
export function makePlainAccount(_initialBalance: number): AccountWithSerializer {
  throw new PendingSolution();
}
