// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.7: make-joint, a second password over one shared account.
 * Pending scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_07.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.7 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Gives newPassword access to account, whose password must match. */
export function makeJoint(
  _account: (password: string, request: JointRequest) => never,
  _password: string,
  _newPassword: string,
): never {
  throw new PendingSolution();
}

/** A request carried to the underlying account. */
export type JointRequest =
  | { readonly _tag: "Withdraw"; readonly amount: number }
  | { readonly _tag: "Deposit"; readonly amount: number };
