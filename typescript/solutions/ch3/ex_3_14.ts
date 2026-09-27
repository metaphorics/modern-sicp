// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { isMNil, type MList, mnil, setCdr } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.14: mystery. The book's inner `loop` reverses the
 * pointers of the input list in place: it walks the chain holding the
 * old tail in a temporary, points each pair's tail back at the
 * accumulator with `setCdr` (the book's `set-cdr!`), and returns the
 * last pair, which is now the front of the reversed chain. The
 * original front ends up as the last pair, its tail the empty list, so
 * the caller's binding prints the one-element list.
 */

/** The book's `mystery`: `x` reversed, by rewriting the pairs' tails
 * in place. The input list is consumed: the caller's binding is left
 * holding its first pair, whose tail is now empty. */
export const mystery = <A>(x: MList<A>): MList<A> => {
  const loop = (rest: MList<A>, acc: MList<A>): MList<A> => {
    if (isMNil(rest)) {
      return acc;
    }
    const temp: MList<A> = rest.tail;
    setCdr(rest, acc);
    return loop(temp, rest);
  };
  return loop(x, mnil);
};
