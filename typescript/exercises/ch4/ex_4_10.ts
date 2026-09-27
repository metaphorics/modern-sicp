// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.10: eval need not be married to one syntax. The demand: use
 * data abstraction to parameterize the evaluator over a syntax table (a
 * map from special-form tag to handler), then install a second syntax for
 * the same language, and show one program evaluate to the same value under
 * both. The pending part is the table, the two installations, and the
 * demonstration.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.10 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The same little program in the standard syntax. */
export const schemeSquareProgram = ["(define square (lambda (x) (* x x)))", "(square 7)"];

/** The same program with every special-form tag spelled backwards. */
export const backwardsSquareProgram = ["(enifed square (adbmal (x) (* x x)))", "(square 7)"];

export function ex_4_10(): string {
  throw new PendingSolution();
}
