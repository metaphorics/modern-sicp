// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.33: with the procedural cons, car, and cdr installed, Ben's
 * literal list construction fails because these lists are ordinary pairs. Modify the
 * evaluator's treatment of literal expressions so literal lists produce true
 * lazy lists.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.33 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_33(): string {
  throw new PendingSolution();
}
