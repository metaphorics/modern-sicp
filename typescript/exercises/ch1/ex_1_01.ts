// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.1: evaluate a sequence of expressions in order.
 *
 * Returns the eleven values the exercise asks for, in the order the
 * expressions appear (the `a === b` response is `false`); the solution
 * replaces the pending throw.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.1 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_1_01(): readonly (number | boolean)[] {
  throw new PendingSolution();
}
