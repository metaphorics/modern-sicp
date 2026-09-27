// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: Chapter 0 section 0.4

/** A fixed-arity product as a readonly tuple: the book's pair. */
export type Point = readonly [number, number];

/** A product with named parts: a readonly record. */
export interface Frac {
  readonly num: number;
  readonly den: number;
}

/** Structural equality for a record, part by part. */
export const equalFrac = (a: Frac, b: Frac): boolean => a.num === b.num && a.den === b.den;

/** The absent-value union: nothing is a value, never `false`, never a throw. */
export type Option<A> = { readonly _tag: "Some"; readonly value: A } | { readonly _tag: "None" };

/** Wraps a present value. */
export const some = <A>(value: A): Option<A> => ({ _tag: "Some", value });

/** The one absent value. */
export const none: Option<never> = { _tag: "None" };

/** A shape: a closed set of variants, known at compile time. */
export type Shape =
  | { readonly _tag: "Circle"; readonly r: number }
  | { readonly _tag: "Rect"; readonly w: number; readonly h: number };

/** The switch must cover every variant; adding one stops the compile. */
export const area = (s: Shape): number => {
  switch (s._tag) {
    case "Circle":
      return Math.PI * s.r * s.r;
    case "Rect":
      return s.w * s.h;
  }
};

/** The empty list; the book's `nil`. */
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
export const car = <A>(l: List<A>): Option<A> => (l._tag === "Cons" ? some(l.head) : none);

/** The rest of a pair, or nothing for the empty list. */
export const cdr = <A>(l: List<A>): Option<List<A>> => (l._tag === "Cons" ? some(l.tail) : none);

/** Structural equality for lists, element by element, with a caller-supplied element equality. */
export const equalList = <A>(a: List<A>, b: List<A>, equal: (x: A, y: A) => boolean): boolean => {
  if (a._tag === "Nil" || b._tag === "Nil") {
    return a._tag === b._tag;
  }
  return equal(a.head, b.head) && equalList(a.tail, b.tail, equal);
};
