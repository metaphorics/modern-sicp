// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import {
  concurrentWorkloadFinal,
  makeAccountDispatchSerialized,
  makeAccountPreSerialized,
  serializedPairFinals,
  unserializedPairFinals,
} from "./ex_3_42.js";

describe("exercise 3.42: serialize once outside dispatch", () => {
  it("both account versions allow exactly the serialized outcome set {60}", () => {
    expect(serializedPairFinals()).toEqual([60]);
  });

  it("without serialization the same workload also allows 80, the lost update", () => {
    expect(unserializedPairFinals()).toEqual([60, 80]);
  });

  it.effect("the dispatch-time and pre-built accounts end the workload at the same balance", () =>
    Effect.gen(function* () {
      const dispatchTime = yield* concurrentWorkloadFinal(makeAccountDispatchSerialized(100));
      const preBuilt = yield* concurrentWorkloadFinal(makeAccountPreSerialized(100));
      // 100 - 20 - 30 + 10 + 5, each operation serialized against the
      // others: 65 either way.
      expect(dispatchTime).toBe(65);
      expect(preBuilt).toBe(65);
    }),
  );

  it.effect("Ben's pre-built account still answers the book's basic sequence", () =>
    Effect.gen(function* () {
      const account = makeAccountPreSerialized(100);
      expect(yield* account.withdraw(25)).toBe(75);
      expect(yield* account.deposit(40)).toBe(115);
      expect(yield* account.balance()).toBe(115);
    }),
  );
});
