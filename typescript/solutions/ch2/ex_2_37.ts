// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import { accumulate, cons, map, nil } from "../../packages/ch2/src/02-picture-language.js";
import { accumulateN } from "./ex_2_36.js";

/**
 * Exercise 2.37: matrix arithmetic as sequence operations. dot-product
 * accumulates entrywise products; the book spells that `(map * v w)`,
 * which the edition's unary map cannot, so a local `map2` walks the two
 * vectors together. Everything else is accumulate, accumulate-n, and map
 * over rows and columns.
 */

/** The book's binary `(map * v w)`, spelled by hand over tagged lists. */
const map2 = (
  f: (x: number, y: number) => number,
  xs: List<number>,
  ys: List<number>,
): List<number> => {
  if (xs._tag === "Nil" || ys._tag === "Nil") {
    return nil;
  }
  return cons(f(xs.head, ys.head), map2(f, xs.tail, ys.tail));
};

/** The sum of the entrywise products of `v` and `w`. */
export const dotProduct = (v: List<number>, w: List<number>): number =>
  accumulate(
    (x, y) => x + y,
    0,
    map2((x, y) => x * y, v, w),
  );

/** Each row of `m` dotted with the vector `v`. */
export const matrixTimesVector = (m: List<List<number>>, v: List<number>): List<number> =>
  map((row) => dotProduct(row, v), m);

/** The columns of `m` become the rows of the result. */
export const transpose = (m: List<List<number>>): List<List<number>> =>
  accumulateN<number, List<number>>(cons, nil, m);

/** Each entry is the dot product of a row of `m` with a column of `n`. */
export const matrixTimesMatrix = (
  m: List<List<number>>,
  n: List<List<number>>,
): List<List<number>> => {
  const cols = transpose(n);
  return map((row) => map((col) => dotProduct(row, col), cols), m);
};
