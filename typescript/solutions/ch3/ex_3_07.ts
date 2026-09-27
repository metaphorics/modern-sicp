// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Result } from "effect";
import type { AccountRequest, InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";

import { IncorrectPassword, type PasswordAccount } from "./ex_3_03.js";

/**
 * Exercise 3.7: make-joint. A joint account is not a second balance
 * but a second password over the first account's Ref: the wrapper
 * translates the new password to the original one and forwards the
 * request, so peter and paul spend the same money. Membership is
 * checked once, at make-joint time, with a balance-neutral probe
 * (a zero deposit): the underlying account answers Incorrect password
 * for a wrong original password, and the book's "in order for the
 * make-joint operation to proceed" becomes that failure.
 */

/** A joint account: same dispatch shape as the password account. */
export type JointAccount = PasswordAccount;

/** Gives `newPassword` access to `account`, whose current password must
 * be `password`. The forwarded requests can still raise
 * `InsufficientFunds`; the membership probe itself cannot, since a
 * zero deposit never overdraws. */
export const makeJoint = (
  account: PasswordAccount,
  password: string,
  newPassword: string,
): Effect.Effect<JointAccount, InsufficientFunds | IncorrectPassword> =>
  Effect.gen(function* () {
    const probe = yield* Effect.result(account(password, { _tag: "Deposit", amount: 0 }));
    if (Result.isFailure(probe)) {
      return yield* probe.failure;
    }
    return (presented, request: AccountRequest) =>
      presented === newPassword ? account(password, request) : Effect.fail(new IncorrectPassword());
  });
