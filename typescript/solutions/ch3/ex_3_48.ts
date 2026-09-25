// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Fiber, type Option } from "effect";

import type { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";
import {
  type AccountWithSerializer,
  exchange,
  type Mutex,
  makeAccountAndSerializer,
  makeMutex,
  type Serializer,
} from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.48: deadlock avoidance by lock ordering. Each account is
 * created with a unique number, and the rewritten serialized exchange
 * always enters the lower-numbered account's serializer first. The
 * argument for why this avoids deadlock is backed by a measured race:
 * two opposing exchanges, with the scheduling window between the two
 * acquisitions made explicit, complete under ordering and deadlock
 * without it.
 */

/** An account carrying its unique number and the mutex its serializer
 * wraps. The mutex travels with the object because the race
 * demonstration needs the two acquire points the serializer otherwise
 * hides. */
export interface NumberedAccount extends AccountWithSerializer {
  readonly accountNumber: number;
  readonly balanceMutex: Mutex;
}

let nextAccountNumber = 1;

/** The book's serializer built over a given mutex, so the numbered
 * account can expose exactly the mutex its serialization uses. */
const serializerFromMutex = (mutex: Mutex): Serializer => {
  return <A, E, R>(p: Effect.Effect<A, E, R>): Effect.Effect<A, E, R> =>
    Effect.gen(function* () {
      yield* mutex.acquire;
      return yield* Effect.ensuring(p, mutex.release);
    });
};

/** The book's modified `make-account`: every account is created with a
 * unique number, here drawn from a process-wide counter, and its
 * serializer wraps a mutex the account can name. */
export const makeNumberedAccount = (initialBalance: number): NumberedAccount => {
  const plain = makeAccountAndSerializer(initialBalance);
  const balanceMutex = makeMutex();
  const accountNumber = nextAccountNumber;
  nextAccountNumber += 1;
  return {
    ...plain,
    serializer: serializerFromMutex(balanceMutex),
    accountNumber,
    balanceMutex,
  };
};

/** The book's rewritten `serialized-exchange`: the account with the
 * smaller number is always serialized outermost, so every process
 * enters its serializers in the same ascending order. */
export const serializedExchangeOrdered = (
  account1: NumberedAccount,
  account2: NumberedAccount,
): Effect.Effect<void, InsufficientFunds> => {
  const outer = account1.accountNumber <= account2.accountNumber ? account1 : account2;
  const inner = outer === account1 ? account2 : account1;
  return Effect.gen(function* () {
    yield* outer.serializer(inner.serializer(exchange(account1, account2)));
  });
};

/** One opposing-exchange program at the mutex level: acquire the
 * outer account, suspend (the scheduling window the hardware has and
 * the single thread must be told about), acquire the inner account,
 * run the exchange, release in reverse. */
const opposingExchange = (
  outer: NumberedAccount,
  inner: NumberedAccount,
  account: NumberedAccount,
  other: NumberedAccount,
): Effect.Effect<void, InsufficientFunds> =>
  Effect.gen(function* () {
    yield* outer.balanceMutex.acquire;
    yield* Effect.yieldNow;
    yield* inner.balanceMutex.acquire;
    yield* exchange(account, other);
    yield* inner.balanceMutex.release;
    yield* outer.balanceMutex.release;
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

/** Peter exchanges a1 with a2 while Paul exchanges a2 with a1, both
 * taking the lower number first. Answers whether the race completes:
 * always true, since a wait cycle would need a descending edge. */
export const orderedRaceCompletes = (
  account1: NumberedAccount,
  account2: NumberedAccount,
): Effect.Effect<boolean> => {
  const lower = account1.accountNumber <= account2.accountNumber ? account1 : account2;
  const higher = lower === account1 ? account2 : account1;
  const peter = opposingExchange(lower, higher, account1, account2);
  const paul = opposingExchange(lower, higher, account2, account1);
  return Effect.map(
    timesOut(Effect.all([peter, paul], { concurrency: 2, discard: true }), 200),
    (timedOut) => !timedOut,
  );
};

/** The same opposing exchanges in the text's nesting order, each
 * process acquiring its own account first. Answers whether the race
 * deadlocks: true, each fiber ends up holding one mutex and waiting
 * forever for the other. */
export const unorderedRaceDeadlocks = (
  account1: NumberedAccount,
  account2: NumberedAccount,
): Effect.Effect<boolean> => {
  const peter = opposingExchange(account1, account2, account1, account2);
  const paul = opposingExchange(account2, account1, account2, account1);
  return timesOut(Effect.all([peter, paul], { concurrency: 2, discard: true }), 200);
};
