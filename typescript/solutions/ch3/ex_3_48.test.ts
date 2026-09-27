// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import {
  makeNumberedAccount,
  type NumberedAccount,
  orderedRaceCompletes,
  serializedExchangeOrdered,
  unorderedRaceDeadlocks,
} from "./ex_3_48.js";

describe("exercise 3.48: deadlock avoidance by lock ordering", () => {
  it("accounts receive ascending unique numbers", () => {
    const first: NumberedAccount = makeNumberedAccount(10);
    const second: NumberedAccount = makeNumberedAccount(20);
    expect(second.accountNumber).toBe(first.accountNumber + 1);
  });

  it.effect("the ordered exchange swaps balances in either argument order", () =>
    Effect.gen(function* () {
      const account1 = makeNumberedAccount(10);
      const account2 = makeNumberedAccount(20);
      yield* serializedExchangeOrdered(account1, account2);
      expect(yield* account1.balance()).toBe(20);
      expect(yield* account2.balance()).toBe(10);
      const account3 = makeNumberedAccount(30);
      const account4 = makeNumberedAccount(40);
      yield* serializedExchangeOrdered(account4, account3);
      expect(yield* account3.balance()).toBe(40);
      expect(yield* account4.balance()).toBe(30);
    }),
  );

  it.effect("concurrent ordered exchanges keep the balances a permutation", () =>
    Effect.gen(function* () {
      const account1 = makeNumberedAccount(10);
      const account2 = makeNumberedAccount(20);
      yield* Effect.all(
        [
          serializedExchangeOrdered(account1, account2),
          serializedExchangeOrdered(account2, account1),
          serializedExchangeOrdered(account1, account2),
        ],
        { concurrency: "unbounded", discard: true },
      );
      const balances = [yield* account1.balance(), yield* account2.balance()].sort((a, b) => a - b);
      expect(balances).toEqual([10, 20]);
    }),
  );

  it("the opposing race with ordering completes", async () => {
    const account1 = makeNumberedAccount(10);
    const account2 = makeNumberedAccount(20);
    const completes = await Effect.runPromise(orderedRaceCompletes(account1, account2));
    expect(completes).toBe(true);
    expect(await Effect.runPromise(account1.balance())).toBe(10);
    expect(await Effect.runPromise(account2.balance())).toBe(20);
  });

  it("the same opposing race without ordering deadlocks", async () => {
    const account1 = makeNumberedAccount(10);
    const account2 = makeNumberedAccount(20);
    const deadlocks = await Effect.runPromise(unorderedRaceDeadlocks(account1, account2));
    expect(deadlocks).toBe(true);
  });
});
