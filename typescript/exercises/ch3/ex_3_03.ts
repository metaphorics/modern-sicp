// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

import type { AccountRequest, InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";

/**
 * Exercise 3.3: password-protected accounts, the section's tagged
 * dispatch behind a password check. Pending scaffold; the solution and
 * its rationale live in solutions/ch3/ex_3_03.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.3 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Builds an account that answers requests only under its password. */
export function makeAccount(
  _initialBalance: number,
  _password: string,
): (password: string, request: AccountRequest) => Effect.Effect<number, InsufficientFunds> {
  throw new PendingSolution();
}
