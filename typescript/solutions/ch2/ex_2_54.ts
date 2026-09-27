// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import type { Datum } from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.54: the book's recursive `equal?`, over the section's
 * `Datum` union. Two data are equal when they are the same symbol or
 * the same number, or when they are lists of equal heads and equal
 * tails.
 */
const itemsEqual = (a: List<Datum>, b: List<Datum>): boolean => {
  if (a._tag === "Nil" || b._tag === "Nil") {
    return a._tag === "Nil" && b._tag === "Nil";
  }
  return equalQ(a.head, b.head) && itemsEqual(a.tail, b.tail);
};

export const equalQ = (x: Datum, y: Datum): boolean => {
  if (x._tag === "Sym" && y._tag === "Sym") {
    return x.name === y.name;
  }
  if (x._tag === "Num" && y._tag === "Num") {
    return x.n === y.n;
  }
  return x._tag === "Lst" && y._tag === "Lst" && itemsEqual(x.items, y.items);
};
