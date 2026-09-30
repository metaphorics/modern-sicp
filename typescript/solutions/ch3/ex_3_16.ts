// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  isMNil,
  type MCons,
  type MList,
  mcons,
  mlist,
  mnil,
} from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.16: Ben's count-pairs. The book's count adds the car
 * count, the cdr count, and one, so a pair reachable by two routes is
 * counted twice and a ring never returns. The builders below are the
 * three three-pair structures of the book's answer: no sharing, one
 * shared pair, and every pair reachable by two routes.
 */

/** Whether a value is a mutable list: a tag check over the section's
 * tagged records, so a nested element can be walked. */
export const isMListValue = (value: unknown): value is MList<unknown> =>
  typeof value === "object" &&
  value !== null &&
  "_tag" in value &&
  (value._tag === "MNil" || value._tag === "MCons");

/** Ben's `count-pairs`: the car count plus the cdr count plus one.
 * Right when nothing is shared; it double counts shared pairs and
 * never returns on a ring. */
export const countPairs = (x: MList<unknown>): number =>
  isMNil(x) ? 0 : (isMListValue(x.head) ? countPairs(x.head) : 0) + countPairs(x.tail) + 1;

/** Three pairs with no sharing, (a b c): Ben's count is right here. */
export const plainX3 = (): MList<unknown> => mlist("a", "b", "c");

/** Three pairs with one shared: ((a) (a)), where both (a) elements
 * are the same pair object. Ben's count returns 4 for 3 pairs. */
export const sharedX2 = (): MList<unknown> => {
  const a = mcons<string>("a", mnil);
  const inner = mcons<MCons<string>>(a, mnil);
  return mcons<MCons<string>>(a, inner);
};

/** Three pairs, each reachable by two routes: the root's car and cdr
 * are the same ((a) (a)) pair, whose own car and cdr are the same (a)
 * pair. Ben's count returns 7 for 3 pairs. */
export const sharedX3 = (): MList<unknown> => {
  const leaf = mcons<string>("a", mnil);
  const mid = mcons<unknown>(leaf, leaf);
  return mcons<unknown>(mid, mid);
};
