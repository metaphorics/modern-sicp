// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.6: let expressions are derived expressions:
 * (let ((v1 e1) ... (vn en)) body) is the same as
 * ((lambda (v1 ... vn) body) e1 ... en). Write the syntactic transformation
 * let->combination that reduces evaluating a let to evaluating a
 * combination, and add the clause to eval.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.6 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_06(): string {
  throw new PendingSolution();
}
