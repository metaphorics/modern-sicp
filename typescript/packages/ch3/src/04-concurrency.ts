// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 3.4

/**
 * Concurrency: time is of the essence, spelled in the edition's idiom.
 * The book's `parallel-execute` forks real fibers; the book's three-step
 * `set!` (access, compute, set) keeps its interleave windows as explicit
 * fiber yields, so a timing diagram like Figure 3.29 can be replayed and
 * its wrong answer reproduced. Serializers are built on the section's own
 * mutex, and the mutex on the book's cell plus an atomic-in-practice
 * `testAndSet` with waiters parked on `Deferred`s: the footnote's "the
 * blocked process is awakened when the mutex becomes available", not a
 * burning spin loop. Accounts are objects over a balance cell, as in the
 * 3.1 module. Because this runtime is single-threaded, two machine steps
 * never interleave by accident; where the text (or an exercise) must
 * enumerate interleavings, the module runs them under an explicit step
 * scheduler: each process is a generator of its book-level steps and a
 * chosen order runs them. Every outcome set this module computes is
 * produced by executing those schedules, never copied.
 */

import { Deferred, Effect, Fiber, Ref } from "effect";

import { InsufficientFunds } from "./01-assignment.js";

// ---------------------------------------------------------------------
// 3.4.1 The Nature of Time in Concurrent Systems
// ---------------------------------------------------------------------

// The section's point is that interleaving order is real and matters:
// the same two withdrawals can leave 65 or a catastrophic 75 depending
// on the order of the steps the timing diagram draws. Single-threaded
// JavaScript never interleaves two plain statements, so the edition
// makes the interleaving explicit: a process is a generator that runs
// one book-level step per resumption, and a scheduler runs chosen
// orders. The steps are the diagram's boxes: access, compute, set.

/** One concurrent process under the step scheduler: a generator that
 * performs its next book-level step on each resumption and yields at
 * the points a timing diagram draws a box boundary. */
export type Process = Generator<void, void, void>;

/** Runs one interleaving: `order[i]` names the process whose next step
 * executes at position `i`. An order that lists each process exactly as
 * many times as it has steps runs every step of every process. */
export const runInterleaving = (
  processes: ReadonlyArray<Process>,
  order: ReadonlyArray<number>,
): void => {
  for (const which of order) {
    processes[which]?.next();
  }
};

/** Every interleaving of processes that take `counts[i]` steps: each
 * order is an array of process indices preserving every process's
 * internal step order. Three processes of three steps give the same
 * 20-with-two-processes arithmetic the section walks through, scaled:
 * here 3 x 3 x 3 orders, all enumerable. */
export const allInterleavings = (
  counts: ReadonlyArray<number>,
): ReadonlyArray<ReadonlyArray<number>> => {
  const results: Array<Array<number>> = [];
  const remaining = [...counts];
  const build = (order: Array<number>): void => {
    if (remaining.every((count) => count <= 0)) {
      results.push(order);
      return;
    }
    for (const which of remaining.keys()) {
      if (remaining[which] === undefined || remaining[which] <= 0) {
        continue;
      }
      remaining[which] -= 1;
      build([...order, which]);
      remaining[which] += 1;
    }
  };
  build([]);
  return results;
};

/** The book's `parallel-execute`: each argument becomes a fiber, all of
 * which run concurrently, and the returned effect joins them. The
 * book's halt handle is the fiber array a caller already holds; Effect
 * fibers carry their own interrupters, so no second control object is
 * minted. */
export const parallelExecute = <A, E>(
  ...ps: ReadonlyArray<Effect.Effect<A, E>>
): Effect.Effect<void, E> =>
  Effect.gen(function* () {
    // Each fork binds to this fiber's scope; forking inside a
    // per-element combinator would close that inner scope as soon as
    // the combinator moves on and interrupt the children before they
    // can be joined.
    const fibers: Array<Fiber.Fiber<A, E>> = [];
    for (const p of ps) {
      fibers.push(yield* Effect.forkChild(p));
    }
    yield* Effect.asVoid(Fiber.joinAll(fibers));
  });

// ---------------------------------------------------------------------
// 3.4.2 Mechanisms for Controlling Concurrency
// ---------------------------------------------------------------------

// The book implements serializers in terms of a mutex, and the mutex in
// terms of a cell (a one-element list) and an atomic test-and-set!. The
// edition keeps that derivation chain. The cell is a mutable record,
// JavaScript object identity makes it shareable, and the single-threaded
// runtime makes the synchronous test-and-set atomic without extra
// machinery: no fiber can run between its read and its write. What the
// book's busy-wait retry loop would burn, this edition parks: a fiber
// that finds the cell held registers a Deferred and suspends, and the
// release wakes the waiters, which is exactly the footnote's operating
// system behavior rather than its `the-mutex 'acquire` loop.

/** The book's cell: a mutable box holding a boolean, `false` when the
 * mutex is available. */
export interface Cell {
  contents: boolean;
}

/** Makes a cell holding `contents`: the book's `(list false)` initial
 * state. */
export const makeCell = (contents: boolean): Cell => ({ contents });

/** The book's `clear!`: marks the cell, and with it the mutex,
 * available. */
export const clearCell = (cell: Cell): void => {
  cell.contents = false;
};

/** The book's `test-and-set!`: tests the cell, and if the test was
 * false, sets the contents to true before answering false. In this
 * runtime the procedure is atomic because nothing can suspend it; the
 * exercises that demonstrate the failure mode inject the window
 * explicitly rather than pretending the plain procedure can fail. */
export const testAndSet = (cell: Cell): boolean => {
  if (cell.contents) {
    return true;
  }
  cell.contents = true;
  return false;
};

/** The book's mutex: it can be acquired and released, and once acquired
 * no other acquire proceeds until release. */
export interface Mutex {
  readonly acquire: Effect.Effect<void>;
  readonly release: Effect.Effect<void>;
}

/** The book's `make-mutex`, minus the burning loop: a waiter that finds
 * the cell true registers on the waiter list and suspends on its
 * Deferred; the release clears the cell and wakes every registered
 * waiter, and the woken retry the test-and-set. Registering happens
 * before suspending, so a release that races a park cannot lose the
 * wakeup. */
export const makeMutex = (): Mutex => {
  const cell = makeCell(false);
  const waiters: Array<Deferred.Deferred<void>> = [];
  const acquire: Effect.Effect<void> = Effect.gen(function* () {
    for (;;) {
      if (!testAndSet(cell)) {
        return;
      }
      const waiter = yield* Deferred.make<void>();
      waiters.push(waiter);
      yield* Deferred.await(waiter);
    }
  });
  const release: Effect.Effect<void> = Effect.gen(function* () {
    clearCell(cell);
    for (const waiter of waiters.splice(0)) {
      yield* Deferred.succeed(waiter, undefined);
    }
  });
  return { acquire, release };
};

/** The book's serializer: applied to a computation, it answers the same
 * computation guarded by its mutex. All applications of one serializer
 * are in the same set: only one runs at a time. */
export type Serializer = <A, E, R>(p: Effect.Effect<A, E, R>) => Effect.Effect<A, E, R>;

/** The book's `make-serializer`: each serializer owns one mutex, and a
 * serialized computation acquires it, runs, and releases. */
export const makeSerializer = (): Serializer => {
  const mutex = makeMutex();
  return <A, E, R>(p: Effect.Effect<A, E, R>): Effect.Effect<A, E, R> =>
    Effect.gen(function* () {
      yield* mutex.acquire;
      return yield* Effect.ensuring(p, mutex.release);
    });
};

// The book account of this section keeps the 3.1.1 shape (withdraw,
// deposit, balance over one encapsulated balance) and adds the
// serializer. The withdraw and deposit run in the book's three steps:
// access the balance, compute the new balance, set it. The step
// boundary between access and set is a fiber yield, so the interleaving
// windows the timing diagrams draw are real scheduling windows here.

/** The bank-account object of this section: withdraw and deposit
 * answer the new balance (or fail with the 3.1 module's typed
 * insufficient-funds error), balance answers the current value. */
export interface Account {
  readonly withdraw: (amount: number) => Effect.Effect<number, InsufficientFunds>;
  readonly deposit: (amount: number) => Effect.Effect<number, InsufficientFunds>;
  readonly balance: () => Effect.Effect<number>;
}

/** A withdraw step sequence over the balance cell: access, test and
 * compute, set, with the interleave window between access and set left
 * open. */
const withdrawSteps = (
  balance: Ref.Ref<number>,
  amount: number,
): Effect.Effect<number, InsufficientFunds> =>
  Effect.gen(function* () {
    const current = yield* Ref.get(balance);
    yield* Effect.yieldNow;
    if (current < amount) {
      return yield* new InsufficientFunds();
    }
    return yield* Ref.setAndGet(balance, current - amount);
  });

/** A deposit step sequence: access, (compute,) set, same open window.
 * The book's footnote is kept honestly: deposit checks nothing, so a
 * negative amount moves money out and can drive the balance below
 * zero. */
const depositSteps = (
  balance: Ref.Ref<number>,
  amount: number,
): Effect.Effect<number, InsufficientFunds> =>
  Effect.gen(function* () {
    const current = yield* Ref.get(balance);
    yield* Effect.yieldNow;
    return yield* Ref.setAndGet(balance, current + amount);
  });

const readBalance = (balance: Ref.Ref<number>): Effect.Effect<number> =>
  Effect.gen(function* () {
    const current = yield* Ref.get(balance);
    yield* Effect.yieldNow;
    return current;
  });

/** The book's `make-account` of this section: deposits and withdrawals
 * are serialized with one per-account serializer, so two processes can
 * never withdraw from or deposit into a single account concurrently.
 * The balance message is deliberately unserialized, as in the text;
 * Exercise 3.41 asks what Ben Bitdiddle thinks of that. */
export const makeAccount = (initialBalance: number): Account => {
  const balance = Ref.makeUnsafe(initialBalance);
  const protectedOperation = makeSerializer();
  return {
    withdraw: (amount) => protectedOperation(withdrawSteps(balance, amount)),
    deposit: (amount) => protectedOperation(depositSteps(balance, amount)),
    balance: () => readBalance(balance),
  };
};

/** The account with its serializer exported: the same internals, but
 * the withdraw and deposit are the raw three-step sequences and the
 * serializer travels with the account, so a user of the object manages
 * serialization explicitly. */
export interface AccountWithSerializer extends Account {
  readonly serializer: Serializer;
}

/** The book's `make-account-and-serializer`. */
export const makeAccountAndSerializer = (initialBalance: number): AccountWithSerializer => {
  const balance = Ref.makeUnsafe(initialBalance);
  const balanceSerializer = makeSerializer();
  return {
    withdraw: (amount) => withdrawSteps(balance, amount),
    deposit: (amount) => depositSteps(balance, amount),
    balance: () => readBalance(balance),
    serializer: balanceSerializer,
  };
};

/** The book's explicit-serializer deposit: fetch the account's
 * serializer and its raw deposit, and run the deposit under the
 * serializer. */
export const deposit = (
  account: AccountWithSerializer,
  amount: number,
): Effect.Effect<number, InsufficientFunds> => account.serializer(account.deposit(amount));

/** The book's `exchange`: read both balances, compute the difference,
 * withdraw it from the first account and deposit it into the second.
 * With individual accounts this can still produce incorrect results:
 * the reads and the updates are not one atomic act. */
export const exchange = (
  account1: Account,
  account2: Account,
): Effect.Effect<void, InsufficientFunds> =>
  Effect.gen(function* () {
    const difference = (yield* account1.balance()) - (yield* account2.balance());
    yield* account1.withdraw(difference);
    yield* account2.deposit(difference);
  });

/** The book's `serialized-exchange`: the whole exchange is serialized
 * with both accounts' serializers, so no other access to either account
 * runs during the exchange. The nesting order is the deadlock the text
 * goes on to discuss: account1's serializer outside, account2's
 * inside. */
export const serializedExchange = (
  account1: AccountWithSerializer,
  account2: AccountWithSerializer,
): Effect.Effect<void, InsufficientFunds> =>
  Effect.gen(function* () {
    yield* account1.serializer(account2.serializer(exchange(account1, account2)));
  });
