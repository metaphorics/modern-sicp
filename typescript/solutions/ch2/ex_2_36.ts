// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import { accumulate, cons, nil } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.36: accumulate-n, which collects the nth elements of a list
 * of lists. The book's car/cdr walk is spelled as two helpers that
 * narrow each list on `_tag`: `heads` takes the first element of every
 * list, `tails` every list after its first, and the recursion stops when
 * the outer list or the first inner list runs out, matching the book's
 * `(null? (car seqs))` guard.
 */

/** The first element of each list; the lists are consumed together. */
const heads = <A>(seqs: List<List<A>>): List<A> => {
  if (seqs._tag === "Nil") {
    return nil;
  }
  const s = seqs.head;
  return s._tag === "Cons" ? cons(s.head, heads(seqs.tail)) : nil;
};

/** Every list without its first element; the lists are consumed together. */
const tails = <A>(seqs: List<List<A>>): List<List<A>> => {
  if (seqs._tag === "Nil") {
    return nil;
  }
  const s = seqs.head;
  return s._tag === "Cons" ? cons(s.tail, tails(seqs.tail)) : nil;
};

/** Accumulates the columns of `seqs` as accumulate accumulates its rows. */
export const accumulateN = <A, B>(op: (x: A, y: B) => B, init: B, seqs: List<List<A>>): List<B> => {
  if (seqs._tag === "Nil" || seqs.head._tag === "Nil") {
    return nil;
  }
  return cons(accumulate(op, init, heads(seqs)), accumulateN(op, init, tails(seqs)));
};
