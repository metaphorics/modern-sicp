// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Ref, Schema } from "effect";

import type { AccountRequest } from "../../packages/ch3/src/01-assignment.js";
import { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";

import { IncorrectPassword } from "./ex_3_03.js";

/**
 * Exercise 3.4: lockout after seven consecutive wrong passwords. The
 * account of exercise 3.3 gains a second local state variable, a Ref
 * counting consecutive wrong-password accesses: a correct access
 * zeroes it, a wrong access increments it, and once the count passes
 * seven the account invokes the book's `call-the-cops` procedure and
 * fails with `CallTheCops` instead of the password complaint.
 */

/** The book's call-the-cops moment, raised as a typed error after the
 * callback has run. */
export class CallTheCops extends Schema.TaggedError<CallTheCops>()(
  "CallTheCops",
  Schema.Struct({}),
) {}

/** Errors a guarded account can raise: the 3.3 pair plus the lockout. */
export type GuardedAccountError = InsufficientFunds | IncorrectPassword | CallTheCops;

/** A password account that calls the cops after seven consecutive bad
 * passwords. */
export type GuardedAccount = (
  password: string,
  request: AccountRequest,
) => Effect.Effect<number, GuardedAccountError>;

/** Builds the exercise 3.3 account with the lockout counter and the
 * `callTheCops` procedure to invoke on the eighth consecutive wrong
 * password. */
export const makeAccount = (
  initialBalance: number,
  password: string,
  callTheCops: () => void,
): GuardedAccount => {
  const encapsulated = Ref.makeUnsafe(initialBalance);
  const consecutiveWrong = Ref.makeUnsafe(0);
  return (presented, request) =>
    Effect.gen(function* () {
      if (presented !== password) {
        const wrong = yield* Ref.updateAndGet(consecutiveWrong, (n) => n + 1);
        if (wrong > 7) {
          callTheCops();
          return yield* new CallTheCops();
        }
        return yield* new IncorrectPassword();
      }
      yield* Ref.set(consecutiveWrong, 0);
      if (request._tag === "Withdraw") {
        const current = yield* Ref.get(encapsulated);
        if (current < request.amount) {
          return yield* new InsufficientFunds();
        }
        return yield* Ref.setAndGet(encapsulated, current - request.amount);
      }
      return yield* Ref.updateAndGet(encapsulated, (b) => b + request.amount);
    });
};
