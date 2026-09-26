// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.16: internal defines bind names that a body's expressions can
 * read before they are assigned. The demand, in three parts: (a) a lookup
 * that fails when it finds the *unassigned* marker; (b) scan-out-defines,
 * which rewrites a procedure body so each internal define becomes a let
 * binding initialized to '*unassigned* plus a set!; (c) the scan installed
 * in the evaluator, choosing make-procedure or procedure-body as the
 * installation point, tested on the book's mutual even?/odd? procedure.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.16 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's f: two internal defines, mutually recursive, plus a call. */
export const fProgram = [
  "(define (f x)",
  "  (define (even? n) (if (= n 0) true (odd? (- n 1))))",
  "  (define (odd? n) (if (= n 0) false (even? (- n 1))))",
  "  (even? x))",
  "(f 7)",
];

export function ex_4_16(): string {
  throw new PendingSolution();
}
