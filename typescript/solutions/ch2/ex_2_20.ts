// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { cons, filter, type List, list } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.20: same-parity takes one argument plus any number more
 * and returns a list of the arguments with the parity of the first.
 * The edition's rest parameter replaces the book's dotted-tail
 * notation, and the module's `filter` carries the parity test the
 * book writes as a nested `cond`.
 */

/** The first argument followed by the rest arguments of its parity. */
export function sameParity(first: number, ...rest: ReadonlyArray<number>): List<number> {
  return cons(
    first,
    filter((x: number) => x % 2 === first % 2, list(...rest)),
  );
}
