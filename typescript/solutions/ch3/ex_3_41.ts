// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Fiber, Ref } from "effect";

import { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";
import {
  type Account,
  makeAccount,
  makeSerializer,
} from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.41: Ben Bitdiddle's serialized balance read. The variant
 * account protects the balance message with the same serializer as
 * withdraw and deposit; the scenario then races one balance read
 * against one in-flight withdrawal on both the text's account and
 * Ben's, so the anomaly (if any) is a measured observation, and the
 * answer falls out of the two runs.
 */

/** Ben's `make-account`: every message, the balance read included,
 * runs under the one per-account serializer. */
export const makeAccountSerializedBalance = (initialBalance: number): Account => {
  const balance = Ref.makeUnsafe(initialBalance);
  const protectedOperation = makeSerializer();
  const withdraw = (amount: number): Effect.Effect<number, InsufficientFunds> =>
    protectedOperation(
      Effect.gen(function* () {
        const current = yield* Ref.get(balance);
        yield* Effect.yieldNow;
        if (current < amount) {
          return yield* new InsufficientFunds();
        }
        return yield* Ref.setAndGet(balance, current - amount);
      }),
    );
  const deposit = (amount: number): Effect.Effect<number, InsufficientFunds> =>
    protectedOperation(
      Effect.gen(function* () {
        const current = yield* Ref.get(balance);
        yield* Effect.yieldNow;
        return yield* Ref.setAndGet(balance, current + amount);
      }),
    );
  return {
    withdraw,
    deposit,
    balance: () => protectedOperation(Ref.get(balance)),
  };
};

/** What one balance read, raced against one in-flight 25 withdrawal,
 * observed. */
export interface ReadDuringUpdateReport {
  readonly readValue: number;
  readonly finalBalance: number;
}

/** Forks the withdrawal and then the balance read together, so the
 * read runs while the withdrawal is in flight, and reports what the
 * read saw and where the account ended. */
export const readDuringWithdraw = (
  account: Account,
): Effect.Effect<ReadDuringUpdateReport, InsufficientFunds> =>
  Effect.gen(function* () {
    const withdrawer = yield* Effect.forkChild(account.withdraw(25));
    const reader = yield* Effect.forkChild(account.balance());
    const readValue = yield* Fiber.join(reader);
    const finalBalance = yield* Fiber.join(withdrawer);
    return { readValue, finalBalance };
  });

/** The run on the text's account: unserialized balance read. */
export const staleReadReport = (): Effect.Effect<ReadDuringUpdateReport, InsufficientFunds> =>
  readDuringWithdraw(makeAccount(100));

/** The run on Ben's account: serialized balance read. */
export const serializedReadReport = (): Effect.Effect<ReadDuringUpdateReport, InsufficientFunds> =>
  readDuringWithdraw(makeAccountSerializedBalance(100));
