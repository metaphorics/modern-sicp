// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.31: product, factorial, and the Wallis formula.
 *
 * `product` mirrors `sum` of 1.3.1 with 1 as the null value and
 * multiplication as the combination. `productIter` is the loop twin: Node
 * gives no tail-call guarantee, so the iterative process is a loop, and it
 * is the twin that survives long ranges (a 50000-term Wallis run overflows
 * the recursion).
 */
export function product(
  term: (x: number) => number,
  a: number,
  next: (x: number) => number,
  b: number,
): number {
  return a > b ? 1 : term(a) * product(term, next(a), next, b);
}

export function productIter(
  term: (x: number) => number,
  a: number,
  next: (x: number) => number,
  b: number,
): number {
  let result = 1;
  let x = a;
  while (x <= b) {
    result = term(x) * result;
    x = next(x);
  }
  return result;
}

/** n! as one call to product: the term is the identity. */
export function factorial(n: number): number {
  return product(
    (x) => x,
    1,
    (x) => x + 1,
    n,
  );
}

/**
 * pi/4 by John Wallis' formula: the product of
 * n(n + 2)/(n + 1)^2 over the even numbers 2, 4, ..., b.
 */
export function wallisPiQuarter(b: number): number {
  const term = (n: number): number => (n * (n + 2)) / ((n + 1) * (n + 1));
  return productIter(term, 2, (n) => n + 2, b);
}
