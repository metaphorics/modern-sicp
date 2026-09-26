// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.2: Louis Reasoner wants the clause for procedure applications
 * to appear before the clause for assignments in eval. (a) Show what goes
 * wrong: under applications-first dispatch, a definition like
 * (define x 3) is itself a pair, so it reaches the application clause and
 * the evaluator tries to apply the symbol define. (b) Change the syntax of
 * the evaluated language instead: every procedure application must begin
 * with the keyword call, as in (call + 1 2), and build the evaluator that
 * dispatches on that form.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.2 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_02(): string {
  throw new PendingSolution();
}
