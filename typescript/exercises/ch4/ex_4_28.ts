// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.28: eval uses actual-value rather than eval on the operator
 * before passing it to apply, to force the operator's value. Give an example
 * that demonstrates the need for this forcing.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.28 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_28(): string {
  throw new PendingSolution();
}
