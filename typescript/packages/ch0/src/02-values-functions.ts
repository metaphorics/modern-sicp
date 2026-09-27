// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: Chapter 0 section 0.2

/** A literal of each kind the book uses: integer, real, truth, text. */
export const planck = 6.626_070_15e-34;

/** A binding names a value; a `const` cannot be reassigned. */
export const size = 2;

/** An expression body writes the computation after the arrow. */
export const square = (x: number): number => x * x;

/** A block body is a braced sequence and returns explicitly. */
export const hypotenuseSquared = (a: number, b: number): number => {
  const a2 = square(a);
  const b2 = square(b);
  return a2 + b2;
};

/** A closure: the returned function keeps `n` alive after the call ends. */
export const makeAdder =
  (n: number): ((x: number) => number) =>
  (x) =>
    x + n;

/** The one-argument function type the section's higher-order listings share. */
export type Mapper = (x: number) => number;

/** Sums `f` over the integers from `a` through `b`, endpoints included. */
export const sumOver = (f: Mapper, a: number, b: number): number => {
  let total = 0;
  for (let k = a; k <= b; k += 1) {
    total += f(k);
  }
  return total;
};

/** Applies `m` twice. */
export const twice =
  (m: Mapper): Mapper =>
  (x) =>
    m(m(x));

/** The conditional expression produces a value: the book's `cond`, in line. */
export const sign = (n: number): string => (n > 0 ? "positive" : n < 0 ? "negative" : "zero");

/** An `if` is a statement: each branch returns explicitly. */
export const classify = (n: number): string => {
  if (n === 0) {
    return "zero";
  }
  if (n >= 1 && n <= 9) {
    return "small";
  }
  return "large";
};

/** Integer division truncates toward zero. */
export const halfOf = (n: number): number => Math.trunc(n / 2);

/** One real operand makes the division real. */
export const realHalf = (n: number): number => n / 2;

/** Exact factorial over `bigint`, for results past the 2^53 boundary. */
export const factorialBig = (n: bigint): bigint => (n <= 1n ? 1n : n * factorialBig(n - 1n));
