// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.22: extend the analyzed evaluator of 4.1.7 with the special
 * form let (exercise 4.6), so a let expression is analyzed into the
 * execution procedure of its lambda combination.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.22 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A let the analyzed evaluator should accept once the clause is added. */
export const letSource = "(let ((x 3)) (+ x 4))";

export function ex_4_22(): string {
  throw new PendingSolution();
}
