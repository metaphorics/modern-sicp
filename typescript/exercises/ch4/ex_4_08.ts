// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.8: named let: (let var bindings body) binds var to a
 * procedure of the binding variables over the body, then calls it with the
 * binding initializers. Modify the let transformation so
 * (let fib ((n 10)) ...) evaluates Fibonacci 10 to 55, following the
 * book's hint to bind the name with set! inside a helper lambda and call
 * it once with a throwaway initial actual.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.8 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_08(): string {
  throw new PendingSolution();
}
