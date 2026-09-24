// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: Chapter 0 section 0.3

/** Factorial as a linear-recursive process: one frame per call. */
export const factorial = (n: number): number => (n <= 1 ? 1 : n * factorial(n - 1));

/** The tree-recursive Fibonacci of section 1.2.2. */
export const fib = (n: number): number => (n < 2 ? n : fib(n - 1) + fib(n - 2));

/** Factorial as an iterative process: the loop carries the state. */
export const factorialIter = (n: number): number => {
  let acc = 1;
  for (let k = 2; k <= n; k += 1) {
    acc *= k;
  }
  return acc;
};

/** Sums the integers 1 through n with a while loop: a constant stack. */
export const sumTo = (n: number): number => {
  let total = 0;
  let k = 1;
  while (k <= n) {
    total += k;
    k += 1;
  }
  return total;
};

/** The naturals as a generator: a recipe, not a row of results. */
export function* naturals(): Generator<number> {
  let n = 1;
  for (;;) {
    yield n;
    n += 1;
  }
}

/** Squares every element of a generator, still deferring the work. */
export function* squaresOf(g: Generator<number>): Generator<number> {
  for (const x of g) {
    yield x * x;
  }
}

/** Drives a generator at most n steps and yields what came out. */
export function* take<A>(g: Generator<A>, n: number): Generator<A> {
  let k = 0;
  for (const x of g) {
    if (k >= n) {
      return;
    }
    yield x;
    k += 1;
  }
}
