// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.16: fast exponentiation by an invariant-carrying loop.
 *
 * The state is (a, b, n) with the product a * b^n constant: halving n
 * squares b, decrementing an odd n moves b into a. When n reaches 0 the
 * answer is a. The loop updates the state variables in place - the
 * iterative shape of this edition, since Node collapses no tail calls.
 */
export const isEven = (n: number): boolean => n % 2 === 0;

export const square = (x: number): number => x * x;

export function fastExptIter(b: number, n: number): number {
  let a = 1;
  let base = b;
  let exp = n;
  while (exp > 0) {
    if (isEven(exp)) {
      base = square(base);
      exp /= 2;
    } else {
      a *= base;
      exp -= 1;
    }
  }
  return a;
}

export function fastExptIterStates(
  b: number,
  n: number,
): Array<{ a: number; b: number; n: number }> {
  const states: Array<{ a: number; b: number; n: number }> = [];
  let a = 1;
  let base = b;
  let exp = n;
  while (exp > 0) {
    states.push({ a, b: base, n: exp });
    if (isEven(exp)) {
      base = square(base);
      exp /= 2;
    } else {
      a *= base;
      exp -= 1;
    }
  }
  states.push({ a, b: base, n: exp });
  return states;
}
