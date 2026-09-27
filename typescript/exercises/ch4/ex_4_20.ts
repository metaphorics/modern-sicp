// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.20: letrec as a derived expression. Part (a) transforms a
 * letrec into a let that pre-binds the names to *unassigned* and then
 * assigns them with set!. Part (b) asks what is loose about Louis's claim
 * that a plain let can replace letrec.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.20 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's f written with letrec: mutually recursive local bindings. */
export const letrecSource = `(define (f x)
  (letrec ((even?
            (lambda (n)
              (if (= n 0) true (odd? (- n 1)))))
           (odd?
            (lambda (n)
              (if (= n 0) false (even? (- n 1))))))
    (even? x)))`;

export function ex_4_20(): string {
  throw new PendingSolution();
}
