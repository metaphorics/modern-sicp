// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Fiber } from "effect";

import type { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";
import {
  type Account,
  type AccountWithSerializer,
  makeAccount,
  makeAccountAndSerializer,
} from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.44: transferring an amount between two accounts needs no
 * joint lock. Ben Bitdiddle's `transfer` is spelled directly over the
 * section's serialized accounts, and its total-conservation property is
 * measured under real concurrent fibers, with a six-transfer workload
 * among three accounts, plus the unserialized contrast that shows what
 * the per-account serialization is doing.
 */

/** The book's `transfer`: withdraw from one account, deposit into the
 * other, with no joint serialization beyond what each account already
 * applies to its own operations. */
export const transfer = (
  fromAccount: Account,
  toAccount: Account,
  amount: number,
): Effect.Effect<void, InsufficientFunds> =>
  Effect.gen(function* () {
    yield* fromAccount.withdraw(amount);
    yield* toAccount.deposit(amount);
  });

const joinVoid = <E>(fiber: Fiber.Fiber<void, E>): Effect.Effect<void, E> => Fiber.join(fiber);

/** A fixed concurrent workload: six transfers among three accounts of
 * 100 each, all forked together. */
const workloadFor = (
  a: Account,
  b: Account,
  c: Account,
): Array<Effect.Effect<void, InsufficientFunds>> => [
  transfer(a, b, 10),
  transfer(b, c, 15),
  transfer(c, a, 5),
  transfer(a, c, 5),
  transfer(b, a, 10),
  transfer(c, b, 15),
];

const totalOf = (a: Account, b: Account, c: Account): Effect.Effect<number> =>
  Effect.gen(function* () {
    const balances = [yield* a.balance(), yield* b.balance(), yield* c.balance()];
    return balances.reduce((sum, balance) => sum + balance, 0);
  });

const runConcurrently = (
  workload: ReadonlyArray<Effect.Effect<void, InsufficientFunds>>,
): Effect.Effect<void, InsufficientFunds> =>
  Effect.gen(function* () {
    // Fork in this fiber's scope so the children survive until joined.
    const fibers: Array<Fiber.Fiber<void, InsufficientFunds>> = [];
    for (const work of workload) {
      fibers.push(yield* Effect.forkChild(work));
    }
    yield* Effect.all(fibers.map((fiber) => joinVoid(fiber)));
  });

/** Runs the workload on serialized accounts and answers the total
 * afterwards: 300, whatever the interleaving. */
export const totalAfterConcurrentTransfers = (): Effect.Effect<number, InsufficientFunds> =>
  Effect.gen(function* () {
    const a = makeAccount(100);
    const b = makeAccount(100);
    const c = makeAccount(100);
    yield* runConcurrently(workloadFor(a, b, c));
    return yield* totalOf(a, b, c);
  });

/** The same workload on raw unserialized accounts: the contrast total,
 * below 300 when the races eat a deposit. */
export const totalAfterUnserializedTransfers = (): Effect.Effect<number, InsufficientFunds> =>
  Effect.gen(function* () {
    const a: AccountWithSerializer = makeAccountAndSerializer(100);
    const b: AccountWithSerializer = makeAccountAndSerializer(100);
    const c: AccountWithSerializer = makeAccountAndSerializer(100);
    yield* runConcurrently(workloadFor(a, b, c));
    return yield* totalOf(a, b, c);
  });
