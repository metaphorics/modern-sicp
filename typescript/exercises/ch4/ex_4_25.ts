// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.25: suppose that in ordinary applicative-order evaluation we define
 * unless as a procedure and then define factorial in terms of it. What happens
 * when we evaluate (factorial 5)? Do the definitions work in a normal-order
 * language?
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.25 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_25(): string {
  throw new PendingSolution();
}
