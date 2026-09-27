// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.32: accumulate, the general combiner.
 *
 * `sum` and `product` are `accumulate` with `+`/0 and `*`/1. The loop twin
 * folds left from `a` - the combiner sees the same arguments in the same
 * order for the associative combiners this chapter uses.
 */
export function accumulate(
  combiner: (x: number, y: number) => number,
  nullValue: number,
  term: (x: number) => number,
  a: number,
  next: (x: number) => number,
  b: number,
): number {
  return a > b
    ? nullValue
    : combiner(term(a), accumulate(combiner, nullValue, term, next(a), next, b));
}

export function accumulateIter(
  combiner: (x: number, y: number) => number,
  nullValue: number,
  term: (x: number) => number,
  a: number,
  next: (x: number) => number,
  b: number,
): number {
  let result = nullValue;
  let x = a;
  while (x <= b) {
    result = combiner(result, term(x));
    x = next(x);
  }
  return result;
}

/** The section's sum, recovered as one call to accumulate. */
export function sumViaAccumulate(
  term: (x: number) => number,
  a: number,
  next: (x: number) => number,
  b: number,
): number {
  return accumulate((x, y) => x + y, 0, term, a, next, b);
}

/** Exercise 1.31's product, recovered as one call to accumulate. */
export function productViaAccumulate(
  term: (x: number) => number,
  a: number,
  next: (x: number) => number,
  b: number,
): number {
  return accumulate((x, y) => x * y, 1, term, a, next, b);
}
