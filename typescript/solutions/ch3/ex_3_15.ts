// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  isMNil,
  type MCons,
  type MList,
  mcons,
  mlist,
  setCar,
} from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.15: setToWow on shared and unshared structure. z1's
 * car and cdr are the very same `[a, b]` pair, so one setCar shows up
 * twice; z2's car and cdr are two distinct copies, so it shows up
 * once.
 */

/** The statement's z1 = `cons(x, x)`: one `[a, b]` pair that the car and the
 * cdr share. The element type is the union because the book's z1 is
 * an improper pair: its tail is a list of atoms, not a list of
 * lists. */
export const makeZ1 = (): MCons<MList<string> | string> => {
  const x = mlist("a", "b");
  return mcons<MList<string> | string>(x, x);
};

/** The statement's z2 = `cons([a, b], [a, b])`: two distinct
 * `[a, b]` pairs at the car and the cdr. */
export const makeZ2 = (): MCons<MList<string> | string> =>
  mcons<MList<string> | string>(mlist("a", "b"), mlist("a", "b"));

/** The book's `setToWow`: setCar of wow on the head pair of
 * `x`, and returns `x` so the caller prints it. */
export const setToWow = (x: MCons<MList<string> | string>): MCons<MList<string> | string> => {
  const head = x.head;
  if (typeof head === "string" || isMNil(head)) {
    throw new Error("setToWow: the head must be a pair");
  }
  setCar(head, "wow");
  return x;
};
