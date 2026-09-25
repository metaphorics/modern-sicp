// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Fiber, type Option, Ref } from "effect";

import { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";
import {
  type AccountWithSerializer,
  deposit,
  makeAccountAndSerializer,
  makeSerializer,
  serializedExchange,
} from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.45: Louis Reasoner's account uses its serializer for
 * withdraw and deposit and exports that very same serializer. The two
 * uses collide: `serialized-exchange` holds the serializer while the
 * exchange's withdraw tries to acquire it again, and the mutex, which
 * is not reentrant, parks the fiber on itself. The demonstrations time
 * real fibers out, so the deadlock is measured, not assumed.
 */

/** Louis's `make-account-and-serializer`: withdraw and deposit are
 * protected, and the serializer message answers that same protector. */
export const makeLouisAccount = (initialBalance: number): AccountWithSerializer => {
  const balance = Ref.makeUnsafe(initialBalance);
  const balanceSerializer = makeSerializer();
  const withdrawSteps = (amount: number): Effect.Effect<number, InsufficientFunds> =>
    Effect.gen(function* () {
      const current = yield* Ref.get(balance);
      yield* Effect.yieldNow;
      if (current < amount) {
        return yield* new InsufficientFunds();
      }
      return yield* Ref.setAndGet(balance, current - amount);
    });
  const depositSteps = (amount: number): Effect.Effect<number, InsufficientFunds> =>
    Effect.gen(function* () {
      const current = yield* Ref.get(balance);
      yield* Effect.yieldNow;
      return yield* Ref.setAndGet(balance, current + amount);
    });
  return {
    withdraw: (amount) => balanceSerializer(withdrawSteps(amount)),
    deposit: (amount) => balanceSerializer(depositSteps(amount)),
    balance: () => Ref.get(balance),
    serializer: balanceSerializer,
  };
};

/** Louis's deposit usage: the account's deposit is already serialized,
 * so the caller just applies it. */
export const louisDeposit = (
  account: AccountWithSerializer,
  amount: number,
): Effect.Effect<number, InsufficientFunds> => account.deposit(amount);

/** Runs `effect` on its own fiber under a real-time limit in
 * milliseconds and answers whether the limit fired: true means the
 * fiber never finished, the signature of a deadlock. */
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

/** Whether a serialized exchange of the two given accounts deadlocks
 * when both are Louis accounts: always true, whichever order. */
export const louisExchangeDeadlocks = (
  account1: AccountWithSerializer,
  account2: AccountWithSerializer,
): Effect.Effect<boolean> => timesOut(serializedExchange(account1, account2), 100);

/** Whether the module's explicit-serializer deposit, which wraps the
 * account's deposit in the exported serializer once more, deadlocks on
 * a Louis account: true, two acquisitions of one non-reentrant
 * serializer on one path. */
export const moduleDepositOnLouisDeadlocks = (
  account: AccountWithSerializer,
): Effect.Effect<boolean> => timesOut(deposit(account, 5), 100);

/** The control: a serialized exchange of plain accounts, whose
 * withdraw and deposit are raw, completes. */
export const plainExchangeCompletes = (
  account1: AccountWithSerializer,
  account2: AccountWithSerializer,
): Effect.Effect<boolean> =>
  Effect.map(timesOut(serializedExchange(account1, account2), 100), (timedOut) => !timedOut);

/** A plain account factory for the controls. */
export const makePlainAccount = makeAccountAndSerializer;
