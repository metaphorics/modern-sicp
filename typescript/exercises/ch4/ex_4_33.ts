// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.33: with the procedural cons, car, and cdr installed, Ben's
 * (car '(a b c)) fails, because quoted lists are ordinary pairs. Modify the
 * evaluator's treatment of quoted expressions so quoted lists produce true
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
