// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.21: recursion without define. Part (a) checks that the
 * self-application expression below computes factorials and asks for an
 * analogous Fibonacci expression. Part (b) asks for the missing operands
 * that complete an even?/odd? procedure built without internal defines or
 * letrec.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.21 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's expression: 10 factorial by passing the procedure to itself. */
export const factorialSource = `((lambda (n)
   ((lambda (fact) (fact fact n))
    (lambda (ft k)
      (if (= k 1)
          1
          (* k (ft ft (- k 1)))))))
 10)`;

/** The skeleton to complete: the two recursion arguments are missing. */
export const evenOddSkeleton = `((lambda (even? odd?)
   (even? even? odd? x))
 (lambda (ev? od? n)
   (if (= n 0) true (od? ?? ?? ??)))
 (lambda (ev? od? n)
   (if (= n 0) false (ev? ?? ?? ??))))`;

export function ex_4_21(): string {
  throw new PendingSolution();
}
