// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import { append, cons, list, map, nil } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.32: the set of all subsets of a set. The subsets of
 * `cons(x, s)` split in two: those omitting `x` are exactly the subsets
 * of `s`; those including it are `x` consed onto each subset of `s`.
 * Computing the subsets of the rest once and mapping the cons over a
 * second pass of that same list produces both halves, in the book's
 * doubling order.
 */

/** All subsets of `s`, in the book's doubling order. */
export const subsets = (s: List<number>): List<List<number>> => {
  if (s._tag === "Nil") {
    return list(nil);
  }
  const rest = subsets(s.tail);
  return append(
    rest,
    map((x) => cons(s.head, x), rest),
  );
};
