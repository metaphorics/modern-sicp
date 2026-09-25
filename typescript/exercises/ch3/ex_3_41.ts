// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

import type { Account } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.41: Ben Bitdiddle's serialized balance read. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_41.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.41 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Ben's `make-account`: every message, the balance read included,
 * runs under the one per-account serializer. */
export function makeAccountSerializedBalance(_initialBalance: number): Account {
  throw new PendingSolution();
}

/** What one balance read, raced against one in-flight 25 withdrawal,
 * observed. */
export interface ReadDuringUpdateReport {
  readonly readValue: number;
  readonly finalBalance: number;
}

/** Forks the balance read and the withdrawal together and reports what
 * the read saw and where the account ended. */
export function readDuringWithdraw(_account: Account): Effect.Effect<ReadDuringUpdateReport> {
  throw new PendingSolution();
}

/** The run on the text's account: unserialized balance read. */
export function staleReadReport(): Effect.Effect<ReadDuringUpdateReport> {
  throw new PendingSolution();
}

/** The run on Ben's account: serialized balance read. */
export function serializedReadReport(): Effect.Effect<ReadDuringUpdateReport> {
  throw new PendingSolution();
}
