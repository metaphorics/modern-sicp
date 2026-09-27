// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Fiber, Ref } from "effect";

import { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";
import {
  type Account,
  allInterleavings,
  makeAccount,
  makeSerializer,
  type Process,
  runInterleaving,
} from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.42: Ben Bitdiddle's pre-serialized account. Instead of
 * wrapping each withdraw/deposit request as the message is answered,
 * the two serialized procedures are built once when the account is
 * made. The question is whether that changes the allowed concurrency;
 * the answer is computed by running the same concurrent workload on
 * both account versions and comparing the outcome sets, with the
 * unserialized raw steps as the contrast.
 */

/** An account constructor, so one workload can run against every
 * account version. */
export type AccountMaker = (initialBalance: number) => Account;

/** Ben's `make-account`: the serialized withdraw and deposit are built
 * once, at account creation, and each request answers that very
 * procedure rather than a fresh wrapping of the message. */
export const makeAccountPreSerialized = (initialBalance: number): Account => {
  const balance = Ref.makeUnsafe(initialBalance);
  const protectedOperation = makeSerializer();
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
  // The book's change: the calls to `protected` happen here, once,
  // outside any dispatch.
  const protectedWithdraw = (amount: number): Effect.Effect<number, InsufficientFunds> =>
    protectedOperation(withdrawSteps(amount));
  const protectedDeposit = (amount: number): Effect.Effect<number, InsufficientFunds> =>
    protectedOperation(depositSteps(amount));
  return {
    withdraw: protectedWithdraw,
    deposit: protectedDeposit,
    balance: () => Ref.get(balance),
  };
};

/** The text's dispatch-time version, under the same maker type. */
export const makeAccountDispatchSerialized = (initialBalance: number): Account =>
  makeAccount(initialBalance);

/** A fixed concurrent workload: two withdrawals and two deposits
 * forked together on one account; answers the final balance. */
export const concurrentWorkloadFinal = (
  account: Account,
): Effect.Effect<number, InsufficientFunds> =>
  Effect.gen(function* () {
    const operations = [
      account.withdraw(20),
      account.withdraw(30),
      account.deposit(10),
      account.deposit(5),
    ];
    const fibers: Array<Fiber.Fiber<number, InsufficientFunds>> = [];
    for (const operation of operations) {
      fibers.push(yield* Effect.forkChild(operation));
    }
    yield* Effect.all(fibers.map((fiber) => Fiber.join(fiber)));
    return yield* account.balance();
  });

/** Final balances of two concurrent 20 withdrawals, each serialized as
 * one atomic chunk, over both orders: the outcome set any correctly
 * serialized account version allows. */
export const serializedPairFinals = (): ReadonlyArray<number> => {
  const outcomes = new Set<number>();
  for (const order of allInterleavings([1, 1])) {
    const state = { balance: 100 };
    const makeWithdraw = (amount: number): Process =>
      // biome-ignore lint/correctness/useYield: a serialized chunk is one scheduler step
      (function* () {
        state.balance -= amount;
      })();
    runInterleaving([makeWithdraw(20), makeWithdraw(20)], order);
    outcomes.add(state.balance);
  }
  return [...outcomes].sort((a, b) => a - b);
};

/** The same two withdrawals on raw two-step withdraws (access, then
 * set), no serialization: what serialization rules out. */
export const unserializedPairFinals = (): ReadonlyArray<number> => {
  const outcomes = new Set<number>();
  for (const order of allInterleavings([2, 2])) {
    const state = { balance: 100 };
    const makeWithdraw = (amount: number): Process =>
      (function* () {
        const accessed = state.balance;
        yield;
        state.balance = accessed - amount;
      })();
    runInterleaving([makeWithdraw(20), makeWithdraw(20)], order);
    outcomes.add(state.balance);
  }
  return [...outcomes].sort((a, b) => a - b);
};
