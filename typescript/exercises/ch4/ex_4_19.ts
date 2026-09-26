// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.19: Ben, Alyssa, and Eva debate the result of the program
 * below. The statement asks which viewpoint (if any) to support and how to
 * implement internal definitions so they behave as Eva prefers.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.19 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The program the three are arguing about. */
export const debatedProgram = `(let ((a 1))
  (define (f x)
    (define b (+ a x))
    (define a 5)
    (+ a b))
  (f 10))`;

export function ex_4_19(): string {
  throw new PendingSolution();
}
