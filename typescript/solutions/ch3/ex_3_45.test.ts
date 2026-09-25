// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import {
  louisDeposit,
  louisExchangeDeadlocks,
  makeLouisAccount,
  makePlainAccount,
  moduleDepositOnLouisDeadlocks,
  plainExchangeCompletes,
} from "./ex_3_45.js";

describe("exercise 3.45: double serialization deadlocks", () => {
  it.live("Louis's own deposit works: one acquisition of the serializer", () =>
    Effect.gen(function* () {
      const account = makeLouisAccount(100);
      expect(yield* louisDeposit(account, 40)).toBe(140);
      expect(yield* account.withdraw(15)).toBe(125);
    }),
  );

  it.live("a serialized exchange of two Louis accounts deadlocks", () =>
    Effect.gen(function* () {
      const account1 = makeLouisAccount(10);
      const account2 = makeLouisAccount(20);
      expect(yield* louisExchangeDeadlocks(account1, account2)).toBe(true);
    }),
  );

  it.live("the deadlock is symmetric in argument order", () =>
    Effect.gen(function* () {
      const account1 = makeLouisAccount(10);
      const account2 = makeLouisAccount(20);
      expect(yield* louisExchangeDeadlocks(account2, account1)).toBe(true);
    }),
  );

  it.live("the module's explicit deposit also deadlocks on a Louis account", () =>
    Effect.gen(function* () {
      const account = makeLouisAccount(100);
      expect(yield* moduleDepositOnLouisDeadlocks(account)).toBe(true);
    }),
  );

  it.live("plain accounts complete the same serialized exchange", () =>
    Effect.gen(function* () {
      const account1 = makePlainAccount(10);
      const account2 = makePlainAccount(20);
      expect(yield* plainExchangeCompletes(account1, account2)).toBe(true);
      expect(yield* account1.balance()).toBe(20);
      expect(yield* account2.balance()).toBe(10);
    }),
  );
});
