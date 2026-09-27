// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { makeAccount } from "./ex_3_03.js";

import { makeJoint } from "./ex_3_07.js";

describe("exercise 3.7: make-joint shares one account", () => {
  it.effect("the joint password reaches peter's balance", () =>
    Effect.gen(function* () {
      const peterAcc = makeAccount(100, "open-sesame");
      const paulAcc = yield* makeJoint(peterAcc, "open-sesame", "rosebud");
      expect(yield* paulAcc("rosebud", { _tag: "Withdraw", amount: 40 })).toBe(60);
    }),
  );

  it.effect("peter observes paul's withdrawals: one shared balance", () =>
    Effect.gen(function* () {
      const peterAcc = makeAccount(100, "open-sesame");
      const paulAcc = yield* makeJoint(peterAcc, "open-sesame", "rosebud");
      yield* paulAcc("rosebud", { _tag: "Withdraw", amount: 40 });
      expect(yield* peterAcc("open-sesame", { _tag: "Withdraw", amount: 60 })).toBe(0);
      yield* peterAcc("open-sesame", { _tag: "Deposit", amount: 30 });
      expect(yield* paulAcc("rosebud", { _tag: "Withdraw", amount: 25 })).toBe(5);
    }),
  );

  it.effect("the joint name rejects its own wrong password", () =>
    Effect.gen(function* () {
      const peterAcc = makeAccount(100, "open-sesame");
      const paulAcc = yield* makeJoint(peterAcc, "open-sesame", "rosebud");
      const failed = yield* Effect.flip(paulAcc("open-sesame", { _tag: "Deposit", amount: 1 }));
      expect(failed._tag).toBe("IncorrectPassword");
    }),
  );

  it.effect("make-joint refuses a wrong original password", () =>
    Effect.gen(function* () {
      const peterAcc = makeAccount(100, "open-sesame");
      const failed = yield* Effect.flip(makeJoint(peterAcc, "wrong-password", "rosebud"));
      expect(failed._tag).toBe("IncorrectPassword");
      expect(yield* peterAcc("open-sesame", { _tag: "Withdraw", amount: 100 })).toBe(0);
    }),
  );
});
