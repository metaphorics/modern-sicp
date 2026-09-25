// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 3.4

import { it } from "@effect/vitest";
import { Effect, Ref } from "effect";
import { describe, expect } from "vitest";

import { InsufficientFunds } from "./01-assignment.js";

import {
  type AccountWithSerializer,
  allInterleavings,
  type Cell,
  clearCell,
  deposit,
  exchange,
  makeAccount,
  makeAccountAndSerializer,
  makeCell,
  makeMutex,
  makeSerializer,
  type Process,
  parallelExecute,
  runInterleaving,
  serializedExchange,
  testAndSet,
} from "./04-concurrency.js";

describe("section 3.4.1: the step scheduler and parallel-execute", () => {
  it("replays the sequential order of Figure 3.29 into 65", () => {
    const bank = { balance: 100 };
    const peter: Process = (function* () {
      const seen = bank.balance;
      yield;
      const next = seen - 10;
      yield;
      bank.balance = next;
    })();
    const paul: Process = (function* () {
      const seen = bank.balance;
      yield;
      const next = seen - 25;
      yield;
      bank.balance = next;
    })();
    runInterleaving([peter, paul], [0, 0, 0, 1, 1, 1]);
    expect(bank.balance).toBe(65);
  });

  it("replays the interleaved order of Figure 3.29 into the catastrophic 75", () => {
    const bank = { balance: 100 };
    const peter: Process = (function* () {
      const seen = bank.balance;
      yield;
      const next = seen - 10;
      yield;
      bank.balance = next;
    })();
    const paul: Process = (function* () {
      const seen = bank.balance;
      yield;
      const next = seen - 25;
      yield;
      bank.balance = next;
    })();
    // Peter accesses, Paul accesses, Peter computes, Paul computes under
    // the stale assumption, Peter sets 90, Paul sets 75.
    runInterleaving([peter, paul], [0, 1, 0, 1, 0, 1]);
    expect(bank.balance).toBe(75);
  });

  it("enumerates exactly the section's 20 orderings of two three-step processes", () => {
    const orders = allInterleavings([3, 3]);
    expect(orders.length).toBe(20);
    for (const order of orders) {
      expect(order.filter((which) => which === 0).length).toBe(3);
      expect(order.filter((which) => which === 1).length).toBe(3);
      // Each process's steps stay in order: the i-th 0 precedes its successors.
      const zeros = order.flatMap((which, index) => (which === 0 ? [index] : []));
      expect([...zeros]).toEqual([...zeros].sort((a, b) => a - b));
    }
  });

  it("interleaving the two withdrawals admits exactly 65, 75, and 90", () => {
    const finals = new Set<number>();
    for (const order of allInterleavings([3, 3])) {
      const bank = { balance: 100 };
      const makeProcess = (amount: number): Process =>
        (function* () {
          const seen = bank.balance;
          yield;
          const next = seen - amount;
          yield;
          bank.balance = next;
        })();
      const peter = makeProcess(10);
      const paul = makeProcess(25);
      runInterleaving([peter, paul], order);
      finals.add(bank.balance);
    }
    expect([...finals].sort((a, b) => a - b)).toEqual([65, 75, 90]);
  });

  it.effect("parallel-execute forks concurrent processes and joins them", () =>
    Effect.gen(function* () {
      const log: Array<string> = [];
      yield* parallelExecute(
        Effect.gen(function* () {
          log.push("peter start");
          yield* Effect.yieldNow;
          log.push("peter end");
        }),
        Effect.gen(function* () {
          log.push("paul start");
          yield* Effect.yieldNow;
          log.push("paul end");
        }),
      );
      // FIFO scheduling interleaves the two fibers at the yield.
      expect(log).toEqual(["peter start", "paul start", "peter end", "paul end"]);
    }),
  );

  it.effect("the book's x example runs to one of its five values, here 100 under FIFO", () =>
    Effect.gen(function* () {
      const x = Ref.makeUnsafe(10);
      const square = Effect.gen(function* () {
        const a = yield* Ref.get(x);
        yield* Effect.yieldNow;
        const b = yield* Ref.get(x);
        yield* Effect.yieldNow;
        return yield* Ref.set(x, a * b);
      });
      const increment = Effect.gen(function* () {
        const c = yield* Ref.get(x);
        yield* Effect.yieldNow;
        return yield* Ref.set(x, c + 1);
      });
      yield* parallelExecute(square, increment);
      expect(yield* Ref.get(x)).toBe(100);
    }),
  );

  it("enumerates exactly the book's five possible values for the x example", () => {
    const finals = new Set<number>();
    for (const order of allInterleavings([3, 2])) {
      const state = { x: 10 };
      const square: Process = (function* () {
        const a = state.x;
        yield;
        const b = state.x;
        yield;
        state.x = a * b;
      })();
      const increment: Process = (function* () {
        const c = state.x;
        yield;
        state.x = c + 1;
      })();
      runInterleaving([square, increment], order);
      finals.add(state.x);
    }
    expect([...finals].sort((a, b) => a - b)).toEqual([11, 100, 101, 110, 121]);
  });
});

describe("section 3.4.2: cells, mutexes, and serializers", () => {
  it("test-and-set! claims a free cell and refuses a taken one; clear! frees it", () => {
    const cell: Cell = makeCell(false);
    expect(testAndSet(cell)).toBe(false);
    expect(cell.contents).toBe(true);
    expect(testAndSet(cell)).toBe(true);
    clearCell(cell);
    expect(cell.contents).toBe(false);
    expect(testAndSet(cell)).toBe(false);
  });

  it.effect("the mutex admits one holder at a time, so the count is exact", () =>
    Effect.gen(function* () {
      const mutex = makeMutex();
      const count = Ref.makeUnsafe(0);
      const work = Effect.gen(function* () {
        for (let i = 0; i < 25; i++) {
          yield* mutex.acquire;
          yield* Ref.updateAndGet(count, (n) => n + 1);
          yield* Effect.yieldNow;
          yield* mutex.release;
        }
      });
      yield* parallelExecute(work, work);
      expect(yield* Ref.get(count)).toBe(50);
    }),
  );

  it.effect("without a mutex the same work loses updates", () =>
    Effect.gen(function* () {
      const count = Ref.makeUnsafe(0);
      const work = Effect.gen(function* () {
        for (let i = 0; i < 25; i++) {
          const seen = yield* Ref.get(count);
          yield* Effect.yieldNow;
          yield* Ref.set(count, seen + 1);
        }
      });
      yield* parallelExecute(work, work);
      // FIFO schedules the two fibers step for step, so every pair of
      // iterations writes the same value once: 25 lost updates.
      expect(yield* Ref.get(count)).toBe(25);
    }),
  );

  it.effect("the serializer's set runs one member at a time: 101 or 121, never less", () =>
    Effect.gen(function* () {
      const s = makeSerializer();
      const x = Ref.makeUnsafe(10);
      const square = s(
        Effect.gen(function* () {
          const a = yield* Ref.get(x);
          yield* Effect.yieldNow;
          const b = yield* Ref.get(x);
          yield* Effect.yieldNow;
          return yield* Ref.set(x, a * b);
        }),
      );
      const increment = s(
        Effect.gen(function* () {
          const c = yield* Ref.get(x);
          yield* Effect.yieldNow;
          return yield* Ref.set(x, c + 1);
        }),
      );
      yield* parallelExecute(square, increment);
      expect(yield* Ref.get(x)).toBe(101);
    }),
  );
});

describe("section 3.4.2: serialized accounts and exchange", () => {
  it.effect("the serialized account answers the book's withdraw sequence", () =>
    Effect.gen(function* () {
      const account = makeAccount(100);
      expect(yield* account.withdraw(25)).toBe(75);
      expect(yield* account.withdraw(25)).toBe(50);
      const failed = yield* Effect.flip(account.withdraw(60));
      expect(failed).toBeInstanceOf(InsufficientFunds);
      expect(yield* account.balance()).toBe(50);
    }),
  );

  it.effect("two serialized withdrawals conserve the balance", () =>
    Effect.gen(function* () {
      const account = makeAccount(100);
      yield* parallelExecute(account.withdraw(10), account.withdraw(25));
      expect(yield* account.balance()).toBe(65);
    }),
  );

  it.effect("the same two withdrawals on raw steps interleave into 75", () =>
    Effect.gen(function* () {
      const account: AccountWithSerializer = makeAccountAndSerializer(100);
      yield* parallelExecute(account.withdraw(10), account.withdraw(25));
      // The raw three-step withdraws race: both read 100, Paul's write
      // lands last under FIFO, the Figure 3.29 anomaly, live.
      expect(yield* account.balance()).toBe(75);
    }),
  );

  it.effect("deposit accepts negative amounts, the footnote's serious bug", () =>
    Effect.gen(function* () {
      const account = makeAccount(10);
      expect(yield* account.deposit(-30)).toBe(-20);
    }),
  );

  it.effect("exchange swaps two balances by moving their difference", () =>
    Effect.gen(function* () {
      const account1 = makeAccount(10);
      const account2 = makeAccount(20);
      yield* exchange(account1, account2);
      expect(yield* account1.balance()).toBe(20);
      expect(yield* account2.balance()).toBe(10);
    }),
  );

  it.effect("the explicit-serializer deposit runs under the account's serializer", () =>
    Effect.gen(function* () {
      const account = makeAccountAndSerializer(100);
      expect(yield* deposit(account, 40)).toBe(140);
    }),
  );

  it.effect("serialized exchanges running concurrently keep the balances' multiset", () =>
    Effect.gen(function* () {
      const account1 = makeAccountAndSerializer(10);
      const account2 = makeAccountAndSerializer(20);
      const account3 = makeAccountAndSerializer(30);
      yield* parallelExecute(
        serializedExchange(account1, account2),
        serializedExchange(account1, account3),
      );
      const balances = [
        yield* account1.balance(),
        yield* account2.balance(),
        yield* account3.balance(),
      ].sort((a, b) => a - b);
      expect(balances).toEqual([10, 20, 30]);
    }),
  );
});
