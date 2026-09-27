// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { makeAccount } from "../../packages/ch3/src/04-concurrency.js";

import {
  totalAfterConcurrentTransfers,
  totalAfterUnserializedTransfers,
  transfer,
} from "./ex_3_44.js";

describe("exercise 3.44: transfer needs no joint lock", () => {
  it.effect("a single transfer moves the amount, Ben's version", () =>
    Effect.gen(function* () {
      const fromAccount = makeAccount(100);
      const toAccount = makeAccount(50);
      yield* transfer(fromAccount, toAccount, 30);
      expect(yield* fromAccount.balance()).toBe(70);
      expect(yield* toAccount.balance()).toBe(80);
    }),
  );

  it.effect("six concurrent transfers among three accounts conserve the total", () =>
    Effect.gen(function* () {
      const total = yield* totalAfterConcurrentTransfers();
      expect(total).toBe(300);
    }),
  );

  it.effect("the same workload on raw unserialized accounts breaks the total", () =>
    Effect.gen(function* () {
      const total = yield* totalAfterUnserializedTransfers();
      // Deterministic FIFO: a stale withdraw write lands after a
      // fresher one, so 10 units the workload spent reappear; the
      // races can as well eat deposits. Either way the total is not
      // the serialized workload's 300.
      expect(total).toBe(310);
    }),
  );
});
