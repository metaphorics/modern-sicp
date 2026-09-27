// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { makeAccount } from "./ex_3_03.js";

describe("exercise 3.3: password-protected accounts", () => {
  it.effect("processes a request under the right password", () =>
    Effect.gen(function* () {
      const acc = makeAccount(100, "secret-password");
      expect(yield* acc("secret-password", { _tag: "Withdraw", amount: 40 })).toBe(60);
      expect(yield* acc("secret-password", { _tag: "Deposit", amount: 50 })).toBe(110);
    }),
  );

  it.effect("complains Incorrect password under the wrong password", () =>
    Effect.gen(function* () {
      const acc = makeAccount(100, "secret-password");
      const failed = yield* Effect.flip(
        acc("some-other-password", { _tag: "Deposit", amount: 50 }),
      );
      expect(failed._tag).toBe("IncorrectPassword");
    }),
  );

  it.effect("a rejected request leaves the balance untouched", () =>
    Effect.gen(function* () {
      const acc = makeAccount(100, "secret-password");
      yield* Effect.flip(acc("some-other-password", { _tag: "Deposit", amount: 50 }));
      expect(yield* acc("secret-password", { _tag: "Withdraw", amount: 100 })).toBe(0);
    }),
  );

  it.effect("each account keeps its own password and balance", () =>
    Effect.gen(function* () {
      const acc = makeAccount(100, "secret-password");
      const acc2 = makeAccount(50, "other-password");
      expect(yield* acc2("other-password", { _tag: "Withdraw", amount: 10 })).toBe(40);
      expect(yield* acc("secret-password", { _tag: "Withdraw", amount: 10 })).toBe(90);
      const failed = yield* Effect.flip(acc("other-password", { _tag: "Withdraw", amount: 10 }));
      expect(failed._tag).toBe("IncorrectPassword");
    }),
  );
});
