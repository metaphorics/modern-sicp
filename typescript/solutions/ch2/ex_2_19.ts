// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  car,
  cdr,
  getOrElse,
  isNull,
  type List,
  list,
  nil,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.19: counting change over a list of coin denominations. The
 * recursion is the book's: the count for an amount splits into the ways
 * that use no more of the first denomination and the ways that use at
 * least one of it. The order of the denomination list does not matter.
 */

export const usCoins: List<number> = list(50, 25, 10, 5, 1);
export const ukCoins: List<number> = list(100, 50, 20, 10, 5, 2, 1, 0.5);

/** Is the denomination list used up? */
export function noMore(coinValues: List<number>): boolean {
  return isNull(coinValues);
}

/** The value of the first denomination; `cc` never calls it on the empty list. */
export function firstDenomination(coinValues: List<number>): number {
  return getOrElse(car(coinValues), 0);
}

/** All denominations but the first. */
export function exceptFirstDenomination(coinValues: List<number>): List<number> {
  return getOrElse(cdr(coinValues), nil);
}

/** The number of ways to make `amount` from the given denominations. */
export function cc(amount: number, coinValues: List<number>): number {
  if (amount === 0) {
    return 1;
  }
  if (amount < 0 || noMore(coinValues)) {
    return 0;
  }
  return (
    cc(amount, exceptFirstDenomination(coinValues)) +
    cc(amount - firstDenomination(coinValues), coinValues)
  );
}
