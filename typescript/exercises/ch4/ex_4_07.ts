// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.7: let* binds sequentially, each binding made in an
 * environment where the previous ones are visible. The book's example
 * (let* ((x 3) (y (+ x 2)) (z (+ x y 5))) (* x z)) returns 39. Write the
 * transformation let*->nested-lets and say whether adding the eval clause
 * (eval (let*->nested-lets exp) env) is enough once let exists.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.7 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_07(): string {
  throw new PendingSolution();
}
