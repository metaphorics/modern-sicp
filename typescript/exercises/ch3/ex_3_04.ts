// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.4: call-the-cops after seven consecutive wrong passwords.
 * Pending scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_04.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.4 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Builds the 3.3 account with a lockout and a callTheCops callback. */
export function makeAccount(
  _initialBalance: number,
  _password: string,
  _callTheCops: () => void,
): never {
  throw new PendingSolution();
}
