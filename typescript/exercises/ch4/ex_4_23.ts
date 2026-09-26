// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.23: compare the text's analyze-sequence, which folds the
 * execution procedures into one at analysis time, with Alyssa's version,
 * which keeps a list and loops through it at run time. The statement asks
 * what work each does for a one-expression body and for a two-expression
 * body.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.23 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Alyssa's expected shape: analyze each expression, loop at run time. */
export const alyssaShapeSource =
  "(define (analyze-sequence exps) (let ((procs (map analyze exps))) (lambda (env) (execute-sequence procs env))))";

export function ex_4_23(): string {
  throw new PendingSolution();
}
