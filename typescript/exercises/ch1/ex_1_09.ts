// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.9: two functions that add two positive integers in terms of
 * inc and dec. inc and plusRecursive are given by the statement; the
 * pending artifact is the instrumented trace of each process.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.9 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Adds one. */
export const inc = (x: number): number => x + 1;

/** Subtracts one. */
export const dec = (x: number): number => x - 1;

/** The recursive shape: the self-call waits inside an inc. */
export const plusRecursive = (a: number, b: number): number =>
  a === 0 ? b : inc(plusRecursive(dec(a), b));

/** The iterative shape: a loop updating the state variables. */
export const plusIterative = (a: number, b: number): number => {
  let x = a;
  let y = b;
  while (x > 0) {
    x = dec(x);
    y = inc(y);
  }
  return y;
};

/** The recursive process's trace: every (a, b) state and the deferred count. */
export function plusRecursiveTrace(
  _a: number,
  _b: number,
): { value: number; states: Array<[number, number]> } {
  throw new PendingSolution();
}

/** The iterative process's trace: the loop's (x, y) state after every pass. */
export function plusIterativeTrace(
  _a: number,
  _b: number,
): { value: number; states: Array<[number, number]> } {
  throw new PendingSolution();
}
