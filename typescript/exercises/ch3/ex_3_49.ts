// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

import type { Mutex } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.49: a scenario where the deadlock-avoidance mechanism of
 * Exercise 3.48 does not work. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_49.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.49 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The shared banking world of the scenario: one ledger and one
 * account, each guarded by its own mutex, and the balance the two
 * resources protect. */
export interface AuditSystem {
  readonly ledger: Mutex;
  readonly accountMutex: Mutex;
  readonly balance: { value: number };
}

/** Makes the scenario's world: balance 100, no holders. */
export function makeAuditSystem(): AuditSystem {
  throw new PendingSolution();
}

/** The transfer clerk: take the ledger (only under it can the clerk
 * read which account the record names), keep it, take the account,
 * move the money, release in reverse. */
export function clerkProgram(_system: AuditSystem): Effect.Effect<void> {
  throw new PendingSolution();
}

/** The auditor: the account is known in advance, so it takes the
 * account first; only after reading the balance does it discover that
 * filing the audit requires the ledger. */
export function auditorProgram(_system: AuditSystem): Effect.Effect<void> {
  throw new PendingSolution();
}

/** The auditor that found the deadlock the hard way and backs off:
 * read under the account lock, release, then take the ledger alone to
 * file. */
export function twoPhaseAuditorProgram(_system: AuditSystem): Effect.Effect<void> {
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

/** Whether clerk and auditor deadlock: true. The clerk holds the
 * ledger and waits for the account; the auditor holds the account and
 * waits for the ledger; numbering cannot order resources a process
 * discovers only after its first acquisition. */
export function clerkAuditorDeadlocks(): Effect.Effect<boolean> {
  throw new PendingSolution();
}

/** The same scenario with the two-phase auditor: it completes, the
 * deadlock-recovery posture the text's footnote points at. */
export function twoPhaseScenarioCompletes(): Effect.Effect<boolean> {
  throw new PendingSolution();
}
