// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.10: the `let` spelling of `make-withdraw` and the binding
 * lifetimes it builds. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_10.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.10 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Builds a withdrawal processor over a balance cell created by the
 * immediately invoked closure that the local binding desugars to. */
export function makeWithdrawLet(_initialAmount: number): never {
  throw new PendingSolution();
}
