// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { type GuardedAccount, makeAccount } from "./ex_3_04.js";

const sevenWarnings = (acc: GuardedAccount) =>
  Effect.gen(function* () {
    for (let i = 0; i < 7; i++) {
      const failed = yield* Effect.flip(acc("wrong-password", { _tag: "Deposit", amount: 1 }));
      expect(failed._tag).toBe("IncorrectPassword");
    }
  });

describe("exercise 3.4: lockout after seven bad passwords", () => {
  it.effect("the first seven wrong accesses still answer Incorrect password", () =>
    Effect.gen(function* () {
      let cops = 0;
      const acc = makeAccount(100, "secret-password", () => {
        cops += 1;
      });
      yield* sevenWarnings(acc);
      expect(cops).toBe(0);
    }),
  );

  it.effect("the eighth consecutive wrong access calls the cops", () =>
    Effect.gen(function* () {
      let cops = 0;
      const acc = makeAccount(100, "secret-password", () => {
        cops += 1;
      });
      yield* sevenWarnings(acc);
      const failed = yield* Effect.flip(acc("wrong-password", { _tag: "Deposit", amount: 1 }));
      expect(failed._tag).toBe("CallTheCops");
      expect(cops).toBe(1);
    }),
  );

  it.effect("a correct access resets the consecutive count", () =>
    Effect.gen(function* () {
      let cops = 0;
      const acc = makeAccount(100, "secret-password", () => {
        cops += 1;
      });
      yield* sevenWarnings(acc);
      expect(yield* acc("secret-password", { _tag: "Deposit", amount: 1 })).toBe(101);
      yield* sevenWarnings(acc);
      expect(cops).toBe(0);
      const failed = yield* Effect.flip(acc("wrong-password", { _tag: "Deposit", amount: 1 }));
      expect(failed._tag).toBe("CallTheCops");
      expect(cops).toBe(1);
    }),
  );

  it.effect("cop calls keep firing while the wrong accesses continue", () =>
    Effect.gen(function* () {
      let cops = 0;
      const acc = makeAccount(100, "secret-password", () => {
        cops += 1;
      });
      yield* sevenWarnings(acc);
      const first = yield* Effect.flip(acc("wrong-password", { _tag: "Deposit", amount: 1 }));
      const second = yield* Effect.flip(acc("wrong-password", { _tag: "Deposit", amount: 1 }));
      expect(first._tag).toBe("CallTheCops");
      expect(second._tag).toBe("CallTheCops");
      expect(cops).toBe(2);
    }),
  );
});
