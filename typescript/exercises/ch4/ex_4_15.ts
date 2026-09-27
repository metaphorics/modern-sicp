// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.15: the halting problem, run rather than argued. Given
 * run-forever and try, suppose halts? correctly decides whether any
 * procedure halts on any input. With a true oracle, (try try) runs
 * forever; with a false oracle, (try (lambda (u) u)) answers 'halted even
 * though that procedure halts on itself. The demand: build both runs and
 * pin what each oracle actually answers.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.15 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's given definitions, spelled in the object language. */
export const runForeverProgram = "(define (run-forever) (run-forever))";

/** The book's try, parameterized over the claimed halts? oracle. */
export const tryProgram = "(define (try p) (if (halts? p p) (run-forever) 'halted))";

export function ex_4_15(): string {
  throw new PendingSolution();
}
