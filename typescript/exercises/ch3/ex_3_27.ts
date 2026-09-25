// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.27: memo-fib, the memoizer's local table seen through
 * the calls it answers. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_27.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.27 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One call the memoizer answered: a `Compute` ran the wrapped
 * lambda for `n` and stored the result in the table, a `Recall`
 * found `n` already there and answered without computing. */
export type MemoFibEvent =
  | { readonly _tag: "Compute"; readonly n: number }
  | { readonly _tag: "Recall"; readonly n: number };

/** The book's memo-fib: the n-th Fibonacci number, each value
 * computed once and recalled from the table ever after. */
export function memoFib(_n: number): number {
  throw new PendingSolution();
}

/** Runs the book's memo-fib and answers the log of the calls it
 * answered. */
export function memoFibTrace(_n: number): MemoFibEvent[] {
  throw new PendingSolution();
}
