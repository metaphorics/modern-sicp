// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.5: Scheme allows one more form of cond clause,
 * (test => recipient). If test evaluates to something other than false,
 * recipient is evaluated; its value must be a procedure of one argument,
 * and that procedure is called on the value of test. Extend the cond
 * expansion so arrow clauses work, and test with an assoc procedure
 * defined in the evaluated language.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.5 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_05(): string {
  throw new PendingSolution();
}
