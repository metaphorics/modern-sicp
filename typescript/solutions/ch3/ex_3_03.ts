// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Ref, Schema } from "effect";

import type { AccountRequest } from "../../packages/ch3/src/01-assignment.js";
import { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";

/**
 * Exercise 3.3: password-protected accounts. The account is the
 * section's tagged dispatch over one Ref, with the book's password
 * argument checked at the dispatch boundary: a request accompanied by
 * the wrong password fails with the book's "Incorrect password"
 * complaint before the balance is touched, and the right password
 * reaches the withdraw/deposit arms unchanged.
 */

/** The book's "Incorrect password" complaint, as a typed error. */
export class IncorrectPassword extends Schema.TaggedError<IncorrectPassword>()(
  "IncorrectPassword",
  Schema.Struct({}),
) {}

/** A password-protected account: password first, then the request. */
export type PasswordAccount = (
  password: string,
  request: AccountRequest,
) => Effect.Effect<number, InsufficientFunds | IncorrectPassword>;

/** Builds an account whose balance starts at `initialBalance` and that
 * answers requests only under `password`. */
export const makeAccount = (initialBalance: number, password: string): PasswordAccount => {
  const encapsulated = Ref.makeUnsafe(initialBalance);
  return (presented, request) =>
    Effect.gen(function* () {
      if (presented !== password) {
        return yield* new IncorrectPassword();
      }
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
