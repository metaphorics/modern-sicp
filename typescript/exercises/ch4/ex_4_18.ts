// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.18: the alternative scan-out strategy. Each define's value
 * expression is computed in an inner let before any set! runs. The
 * statement asks whether the solve procedure of 3.5.4 works under this
 * scan, and whether it works under the text's scan, and why.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.18 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's solve procedure, whose value expressions read each other. */
export const solveSource = `((lambda (f y0 dt)
  (define y (integral (delay dy) y0 dt))
  (define dy (stream-map f y))
  y) (lambda (x) x) 1 0.1)`;

/** The alternative's shape: values computed up front, assigned after. */
export const alternativeSource =
  "(let ((u '*unassigned*) (v '*unassigned*)) (let ((a e1) (b e2)) (set! u a) (set! v b)) e3)";

export function ex_4_18(): string {
  throw new PendingSolution();
}
