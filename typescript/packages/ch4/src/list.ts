// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

/**
 * The cons list. Effect v4 ships no persistent `List` module, so the edition
 * hand-writes the book's pairs as a discriminated union: sections 0.4 and 2.2
 * introduce this shape in their listings, and the chapter 4 evaluator uses it
 * for quoted data and parameter lists.
 */
import { Option } from "effect";

/** The empty list, `nil` in the book. */
export interface Nil {
  readonly _tag: "Nil";
}

/** A cons cell: a head value and the rest of the list. */
export interface Cons<A> {
  readonly _tag: "Cons";
  readonly head: A;
  readonly tail: List<A>;
}

export type List<A> = Nil | Cons<A>;

/** The empty list. */
export const nil: List<never> = { _tag: "Nil" };

/** Pairs `head` onto an existing `tail`. */
export const cons = <A>(head: A, tail: List<A>): List<A> => ({ _tag: "Cons", head, tail });

/** Builds a list from a sequence, preserving left-to-right order. */
export const list = <A>(...items: ReadonlyArray<A>): List<A> =>
  items.reduceRight<List<A>>((tail, head) => cons(head, tail), nil);

/** The first element of a pair, or nothing for the empty list. */
export const car = <A>(l: List<A>): Option.Option<A> =>
  l._tag === "Cons" ? Option.some(l.head) : Option.none();

/** The rest of a pair, or nothing for the empty list. */
export const cdr = <A>(l: List<A>): Option.Option<List<A>> =>
  l._tag === "Cons" ? Option.some(l.tail) : Option.none();

/** The elements as a host array, left to right. */
export const toArray = <A>(l: List<A>): ReadonlyArray<A> => {
  const out: A[] = [];
  let rest = l;
  while (rest._tag === "Cons") {
    out.push(rest.head);
    rest = rest.tail;
  }
  return out;
};
