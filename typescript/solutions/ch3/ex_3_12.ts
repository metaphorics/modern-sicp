// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  isMNil,
  type MCons,
  type MList,
  mcons,
  mlist,
  setCdr,
  showMList,
} from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.12: append versus append!. The book's `append` conses a
 * fresh chain for `x` and ends it at the very `y` object, so `x` is
 * unchanged; the book's `append!` rewrites the final pair of `x` so
 * the two lists share one spine, and `x` is changed. Identity is
 * JavaScript object identity, so the demo can show exactly which
 * pairs the two versions share.
 */

/** The book's `append` over mutable pairs: a fresh copy of `x`
 * followed by the very `y` object, leaving `x` unchanged. */
export const append = <A>(x: MList<A>, y: MList<A>): MList<A> =>
  isMNil(x) ? y : mcons(x.head, append(x.tail, y));

/** The book's `last-pair`: the final pair of a nonempty list. The
 * book only calls it on nonempty lists; the empty case throws. */
export const lastPair = <A>(x: MList<A>): MCons<A> => {
  if (isMNil(x)) {
    throw new Error("last-pair: empty list");
  }
  return isMNil(x.tail) ? x : lastPair(x.tail);
};

/** The book's `append!`: splices `y` onto `x` by rewriting the final
 * pair of `x` in place, and returns `x`. The book calls it an error
 * to pass an empty `x`; the empty case throws. */
export const appendBang = <A>(x: MList<A>, y: MList<A>): MList<A> => {
  if (isMNil(x)) {
    throw new Error("append!: empty first list");
  }
  setCdr(lastPair(x), y);
  return x;
};

/** The four printed shapes of the book's interaction, plus the
 * identity pin: what `z`, `(cdr x)`, `w`, and `(cdr x)` show after
 * each append, and whether the final pair of `x` now tails onto `y`
 * itself. */
export interface AppendDemo {
  readonly zAfterAppend: string;
  readonly cdrXAfterAppend: string;
  readonly wAfterAppendBang: string;
  readonly cdrXAfterAppendBang: string;
  readonly xTailIsY: boolean;
}

/** Runs the book's interaction over x = (a b), y = (c d): `z =
 * (append x y)`, inspect `(cdr x)`, then `w = (append! x y)` and
 * inspect again. */
export const appendDemo = (): AppendDemo => {
  const x = mlist("a", "b");
  const y = mlist("c", "d");
  if (isMNil(x)) {
    throw new Error("append demo: x must be nonempty");
  }
  const z = append(x, y);
  const cdrXAfterAppend = showMList(x.tail);
  const xFinalPair = lastPair(x);
  const w = appendBang(x, y);
  return {
    zAfterAppend: showMList(z),
    cdrXAfterAppend,
    wAfterAppendBang: showMList(w),
    cdrXAfterAppendBang: showMList(x.tail),
    xTailIsY: Object.is(xFinalPair.tail, y),
  };
};
