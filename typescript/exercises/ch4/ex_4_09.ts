// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.9: many languages offer iteration constructs such as do, for,
 * while, and until, while Scheme expresses iteration with ordinary
 * procedure calls. The demand: design and implement iteration constructs
 * for the evaluator as derived expressions (or otherwise), without
 * cheating through make-procedure or the host's apply. The pending part is
 * the constructs themselves plus evidence that they run iterative
 * processes, here summing loops checked against manual recursion.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.9 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A while loop that accumulates 1 + 2 + 3 + 4 into total. */
export const summingWhileProgram = [
  "(define total 0)",
  "(define i 1)",
  "(while (< i 5) (set! total (+ total i)) (set! i (+ i 1)))",
  "total",
];

/** A for loop that accumulates 1 through 5 into total. */
export const summingForProgram = [
  "(define total 0)",
  "(for (n 1 5) (set! total (+ total n)))",
  "total",
];

/** The same sum written as manual recursion, the standard it must match. */
export const manualSumProgram = [
  "(define (sum-to n) (if (= n 0) 0 (+ n (sum-to (- n 1)))))",
  "(sum-to 5)",
];

export function ex_4_09(): string {
  throw new PendingSolution();
}
