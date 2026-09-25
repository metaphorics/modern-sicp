// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 3.1

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";
import {
  cesaroTest,
  estimatePi,
  estimatePiWithoutState,
  factorialImperative,
  InsufficientFunds,
  makeAccount,
  makeDecrementer,
  makeRand,
  makeRandomLive,
  makeSimplifiedWithdraw,
  makeWithdraw,
  monteCarlo,
  newWithdraw,
  Random,
  randomGcdTest,
  randomInit,
  withdraw,
} from "./01-assignment.js";

describe("section 3.1: assignment and local state", () => {
  it.effect("withdraw over the global balance answers the book's sequence", () =>
    Effect.gen(function* () {
      expect(yield* withdraw(25)).toBe(75);
      expect(yield* withdraw(25)).toBe(50);
      const failed = yield* Effect.flip(withdraw(60));
      expect(failed).toBeInstanceOf(InsufficientFunds);
      expect(yield* withdraw(15)).toBe(35);
    }),
  );

  it.effect("new-withdraw encapsulates its balance", () =>
    Effect.gen(function* () {
      const w = newWithdraw();
      expect(yield* w(50)).toBe(50);
      const failed = yield* Effect.flip(w(60));
      expect(failed._tag).toBe("InsufficientFunds");
    }),
  );

  it.effect("make-withdraw objects are independent, as in the book", () =>
    Effect.gen(function* () {
      const w1 = makeWithdraw(100);
      const w2 = makeWithdraw(100);
      expect(yield* w1(50)).toBe(50);
      expect(yield* w2(70)).toBe(30);
      const failed = yield* Effect.flip(w2(40));
      expect(failed._tag).toBe("InsufficientFunds");
      expect(yield* w1(40)).toBe(10);
    }),
  );

  it.effect("make-account answers the book's interaction sequence", () =>
    Effect.gen(function* () {
      const acc = makeAccount(100);
      expect(yield* acc({ _tag: "Withdraw", amount: 50 })).toBe(50);
      const failed = yield* Effect.flip(acc({ _tag: "Withdraw", amount: 60 }));
      expect(failed._tag).toBe("InsufficientFunds");
      expect(yield* acc({ _tag: "Deposit", amount: 40 })).toBe(90);
      expect(yield* acc({ _tag: "Withdraw", amount: 60 })).toBe(30);
      const acc2 = makeAccount(100);
      expect(yield* acc2({ _tag: "Withdraw", amount: 10 })).toBe(90);
      expect(yield* acc({ _tag: "Withdraw", amount: 10 })).toBe(20);
    }),
  );

  it("peter and paul sharing one object is aliasing, not two accounts", () => {
    const peterAcc = makeAccount(100);
    const paulAcc = peterAcc;
    expect(Effect.runSync(paulAcc({ _tag: "Withdraw", amount: 30 }))).toBe(70);
    expect(Effect.runSync(peterAcc({ _tag: "Withdraw", amount: 30 }))).toBe(40);
    const separatePaul = makeAccount(100);
    expect(Effect.runSync(separatePaul({ _tag: "Withdraw", amount: 30 }))).toBe(70);
  });

  it.effect("rand-update chains from randomInit exactly as pinned", () =>
    Effect.gen(function* () {
      const rand = makeRand(randomInit);
      expect(yield* rand).toBe(11355432);
      expect(yield* rand).toBe(2836018348);
      expect(yield* rand).toBe(476557059);
      expect(yield* rand).toBe(3648046016);
    }),
  );

  it.effect("the Random service is a reproducible generator", () =>
    Effect.gen(function* () {
      const random = yield* Random;
      expect(yield* random.next).toBe(11355432);
      expect(yield* random.next).toBe(2836018348);
    }).pipe(Effect.provide(makeRandomLive(randomInit))),
  );

  it.effect("cesaro-test and monte-carlo estimate pi at 10000 trials", () =>
    Effect.gen(function* () {
      const fraction = yield* monteCarlo(10000, cesaroTest);
      expect(fraction).toBeGreaterThan(0.55);
      expect(fraction).toBeLessThan(0.67);
      const pi = yield* estimatePi(10000);
      expect(Math.abs(pi - Math.PI)).toBeLessThan(0.1);
    }).pipe(Effect.provide(makeRandomLive(randomInit))),
  );

  it.effect("the stateful service and the stateless threading walk the same chain", () =>
    Effect.gen(function* () {
      expect(randomGcdTest(10000, randomInit)).toBe(0.6105);
      expect(estimatePiWithoutState(10000)).toBeCloseTo(3.1349656821103844, 10);
      const pi = yield* estimatePi(10000);
      expect(pi).toBe(estimatePiWithoutState(10000));
    }).pipe(Effect.provide(makeRandomLive(randomInit))),
  );

  it.effect("make-simplified-withdraw accumulates into negatives", () =>
    Effect.gen(function* () {
      const w = makeSimplifiedWithdraw(25);
      expect(yield* w(20)).toBe(5);
      expect(yield* w(10)).toBe(-5);
    }),
  );

  it.effect("make-decrementer answers the same value every call", () =>
    Effect.gen(function* () {
      const d = makeDecrementer(25);
      expect(yield* d(20)).toBe(5);
      expect(yield* d(10)).toBe(15);
      expect(yield* d(20)).toBe(5);
    }),
  );

  it.effect("the imperative factorial matches the recursive book values", () =>
    Effect.gen(function* () {
      expect(yield* factorialImperative(5)).toBe(120);
      expect(yield* factorialImperative(10)).toBe(3628800);
      expect(yield* factorialImperative(1)).toBe(1);
      expect(yield* factorialImperative(0)).toBe(1);
    }),
  );
});
