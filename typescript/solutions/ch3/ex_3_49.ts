// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Fiber, type Option } from "effect";

import { type Mutex, makeMutex } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.49: a scenario where the deadlock-avoidance mechanism of
 * Exercise 3.48 does not work. A transfer clerk must take the ledger
 * lock first, because the transfer record naming the account to move
 * is only readable under it; a balance auditor is assigned its account
 * in advance and only discovers, while holding the account lock, that
 * filing the audit requires the ledger. Neither process can name its
 * full resource set when it starts acquiring, so there is no global
 * order both can follow, and the numbered-account scheme cannot be
 * extended to help. The deadlock is measured; the two-phase variant
 * shows what recovery looks like.
 */

/** The shared banking world of the scenario: one ledger and one
 * account, each guarded by its own mutex, and the balance the two
 * resources protect. */
export interface AuditSystem {
  readonly ledger: Mutex;
  readonly accountMutex: Mutex;
  readonly balance: { value: number };
}

/** Makes the scenario's world: balance 100, no holders. */
export const makeAuditSystem = (): AuditSystem => ({
  ledger: makeMutex(),
  accountMutex: makeMutex(),
  balance: { value: 100 },
});

/** The transfer clerk: take the ledger (only under it can the clerk
 * read which account the record names), keep it, take the account,
 * move the money, release in reverse. */
export const clerkProgram = (system: AuditSystem): Effect.Effect<void> =>
  Effect.gen(function* () {
    yield* system.ledger.acquire;
    yield* Effect.yieldNow;
    yield* system.accountMutex.acquire;
    yield* Effect.yieldNow;
    system.balance.value = system.balance.value - 25;
    yield* system.accountMutex.release;
    yield* system.ledger.release;
  });

/** The auditor: the account is known in advance, so it takes the
 * account first; only after reading the balance does it discover that
 * filing the audit requires the ledger. */
export const auditorProgram = (system: AuditSystem): Effect.Effect<void> =>
  Effect.gen(function* () {
    yield* system.accountMutex.acquire;
    yield* Effect.yieldNow;
    const observed = system.balance.value;
    yield* system.ledger.acquire;
    yield* Effect.yieldNow;
    // Filing needs the ledger while the account is still held: the
    // audit record must name the balance it read.
    system.balance.value = observed;
    yield* system.ledger.release;
    yield* system.accountMutex.release;
  });

/** The auditor that found the deadlock the hard way and backs off:
 * read under the account lock, release, then take the ledger alone to
 * file. The needs were only knowable in stages, and the second stage
 * no longer holds the first resource. */
export const twoPhaseAuditorProgram = (system: AuditSystem): Effect.Effect<void> =>
  Effect.gen(function* () {
    yield* system.accountMutex.acquire;
    yield* Effect.yieldNow;
    const observed = system.balance.value;
    yield* system.accountMutex.release;
    yield* system.ledger.acquire;
    yield* Effect.yieldNow;
    system.balance.value = observed;
    yield* system.ledger.release;
  });

/** Runs `effect` on its own fiber under a real-time limit in
 * milliseconds and answers whether the limit fired. */
export const timesOut = <A, E>(
  effect: Effect.Effect<A, E>,
  millis: number,
): Effect.Effect<boolean> =>
  Effect.gen(function* () {
    const fiber = yield* Effect.forkChild(effect);
    const outcome: Option.Option<A> = yield* Effect.option(
      Effect.timeout(`${millis} millis`)(Fiber.join(fiber)),
    );
    return outcome._tag === "None";
  });

/** Whether clerk and auditor deadlock: true. The clerk holds the
 * ledger and waits for the account; the auditor holds the account and
 * waits for the ledger; numbering cannot order resources a process
 * discovers only after its first acquisition. */
export const clerkAuditorDeadlocks = (): Effect.Effect<boolean> => {
  const system = makeAuditSystem();
  return timesOut(
    Effect.all([clerkProgram(system), auditorProgram(system)], {
      concurrency: 2,
      discard: true,
    }),
    200,
  );
};

/** The same scenario with the two-phase auditor: it completes, the
 * deadlock-recovery posture the text's footnote points at. */
export const twoPhaseScenarioCompletes = (): Effect.Effect<boolean> => {
  const system = makeAuditSystem();
  return Effect.map(
    timesOut(
      Effect.all([clerkProgram(system), twoPhaseAuditorProgram(system)], {
        concurrency: 2,
        discard: true,
      }),
      200,
    ),
    (timedOut) => !timedOut,
  );
};
