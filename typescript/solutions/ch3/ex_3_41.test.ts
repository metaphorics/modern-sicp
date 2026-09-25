// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import {
  makeAccountSerializedBalance,
  readDuringWithdraw,
  serializedReadReport,
  staleReadReport,
} from "./ex_3_41.js";

describe("exercise 3.41: should balance reads serialize", () => {
  it.effect("the text's unserialized read observes 100 while the 25 withdrawal is in flight", () =>
    Effect.gen(function* () {
      const report = yield* staleReadReport();
      // FIFO: the withdrawal accesses 100 and yields inside its
      // three-step window; the reader reads the same 100 there, then
      // the withdrawal lands 75. The snapshot is stale the moment the
      // withdrawal commits, and no money is lost.
      expect(report.readValue).toBe(100);
      expect(report.finalBalance).toBe(75);
    }),
  );

  it.effect("Ben's serialized read waits for the withdrawal and observes 75", () =>
    Effect.gen(function* () {
      const report = yield* serializedReadReport();
      expect(report.readValue).toBe(75);
      expect(report.finalBalance).toBe(75);
    }),
  );

  it.effect("the serialized balance still serializes with withdrawals on Ben's account", () =>
    Effect.gen(function* () {
      const account = makeAccountSerializedBalance(100);
      expect(yield* account.withdraw(10)).toBe(90);
      expect(yield* account.balance()).toBe(90);
      expect(yield* account.deposit(5)).toBe(95);
      expect(yield* account.balance()).toBe(95);
    }),
  );

  it.effect("the readDuringWithdraw harness on a fresh Ben account mirrors the pins", () =>
    Effect.gen(function* () {
      const report = yield* readDuringWithdraw(makeAccountSerializedBalance(100));
      expect(report).toEqual({ readValue: 75, finalBalance: 75 });
    }),
  );
});
