// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { append, cons, type List, nil } from "../../packages/ch2/src/02-picture-language.js";
import { elementOfSetUnordered } from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.60: the duplicate-allowed representation (a bag). Membership
 * still scans until it hits the element, adjoining is a single cons,
 * union is an append, and intersection keeps the elements of set1 that
 * set2 also holds. Operation counts against the no-duplicate
 * representation are in the rationale.
 */

/** Same membership as before: Theta(n) in the worst case; duplicates
 * only make an early hit likelier. */
export const elementOfBag = (x: number, set: List<number>): boolean =>
  elementOfSetUnordered(x, set);

/** Theta(1): no membership check, so duplicates accumulate. */
export const adjoinBag = (x: number, set: List<number>): List<number> => cons(x, set);

/** Theta(n1 + n2): concatenate; duplicates survive. */
export const unionBag = (set1: List<number>, set2: List<number>): List<number> =>
  append(set1, set2);

/** Theta(n1 * n2): for every element of set1, scan all of set2; the
 * result keeps set1's multiplicities. */
export const intersectionBag = (set1: List<number>, set2: List<number>): List<number> => {
  if (set1._tag === "Nil" || set2._tag === "Nil") {
    return nil;
  }
  return elementOfBag(set1.head, set2)
    ? cons(set1.head, intersectionBag(set1.tail, set2))
    : intersectionBag(set1.tail, set2);
};
