// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 2.1

// Wishful thinking (2.1.1): rational-number arithmetic on three
// yet-unspecified procedures.

/**
 * A rational number as a numerator/denominator pair: the book's rational
 * number, on this edition's exact integers.
 */
export type Rat = readonly [numer: bigint, denom: bigint];

/** Adds n1/d1 and n2/d2 as (n1 d2 + n2 d1)/(d1 d2): the book's add-rat. */
export const addRat = (x: Rat, y: Rat): Rat => makeRat(x[0] * y[1] + y[0] * x[1], x[1] * y[1]);

/** Subtracts n2/d2 from n1/d1 as (n1 d2 - n2 d1)/(d1 d2): the book's sub-rat. */
export const subRat = (x: Rat, y: Rat): Rat => makeRat(x[0] * y[1] - y[0] * x[1], x[1] * y[1]);

/** Multiplies as (n1 n2)/(d1 d2): the book's mul-rat. */
export const mulRat = (x: Rat, y: Rat): Rat => makeRat(x[0] * y[0], x[1] * y[1]);

/** Divides as (n1 d2)/(d1 n2): the book's div-rat. */
export const divRat = (x: Rat, y: Rat): Rat => makeRat(x[0] * y[1], x[1] * y[0]);

/** Tests n1 d2 = n2 d1: the book's equal-rat?. */
export const equalRat = (x: Rat, y: Rat): boolean => x[0] * y[1] === y[0] * x[1];

// Pairs (2.1.1).

/** Glues two values into the host's pair, the readonly two-element tuple: the book's cons. */
export const cons = <A, B>(x: A, y: B): readonly [A, B] => [x, y];

/** The first element of a pair: the book's car. */
export const car = <A, B>(p: readonly [A, B]): A => p[0];

/** The second element of a pair: the book's cdr. */
export const cdr = <A, B>(p: readonly [A, B]): B => p[1];

// Representing rational numbers (2.1.1).

/** Euclid's gcd over exact integers, on absolute values: the 1.2.5 algorithm. */
const gcd = (a: bigint, b: bigint): bigint => {
  let x = a < 0n ? -a : a;
  let y = b < 0n ? -b : b;
  while (y !== 0n) {
    const r = x % y;
    x = y;
    y = r;
  }
  return x;
};

/**
 * Builds a rational number in lowest terms: the book's make-rat after the
 * gcd fix. The denominator must be nonzero, per the 2.1.3 condition; the
 * sign is not yet normalized, which is exercise 2.1's work.
 */
export const makeRat = (n: bigint, d: bigint): Rat => {
  const g = gcd(n, d);
  return [n / g, d / g];
};

/** The subsection's first spelling of make-rat: the bare pair, no reduction. */
export const makeRatUnreduced = (n: bigint, d: bigint): Rat => cons(n, d);

/** The numerator: the book's numer. */
export const numer = (x: Rat): bigint => x[0];

/** The denominator: the book's denom. */
export const denom = (x: Rat): bigint => x[1];

/** Renders the numerator, a slash, and the denominator: the book's print-rat. */
export const printRat = (x: Rat): string => `${numer(x)}/${denom(x)}`;

// Abstraction barriers (2.1.2): the same contract with the reduction at
// access time.

/** The alternate representation's constructor: the bare pair again. */
export const makeRatAtAccess = (n: bigint, d: bigint): Rat => cons(n, d);

/** Divides out the gcd when the numerator is selected: the alternate numer. */
export const numerAtAccess = (x: Rat): bigint => {
  const g = gcd(x[0], x[1]);
  return x[0] / g;
};

/** Divides out the gcd when the denominator is selected: the alternate denom. */
export const denomAtAccess = (x: Rat): bigint => {
  const g = gcd(x[0], x[1]);
  return x[1] / g;
};

// What is meant by data (2.1.3): the two-sided value of 0.6.

/** The book's two-sided result: a value or an error, never a throw. */
export type Result<A, E> =
  | { readonly _tag: "Ok"; readonly value: A }
  | { readonly _tag: "Error"; readonly error: E };

/** Wraps a success value. */
export const ok = <A>(value: A): Result<A, never> => ({ _tag: "Ok", value });

/** Wraps a failure value. */
export const err = <E>(error: E): Result<never, E> => ({ _tag: "Error", error });

/** The dispatch failure of the procedural pair: the book's "Argument not 0 or 1: CONS". */
export type PairMessageError = { readonly _tag: "UnknownMessage"; readonly m: number };

/**
 * A pair built from procedures alone: a dispatch that answers message 0
 * with the first part and message 1 with the second.
 */
export type ProcPair<T> = (m: number) => Result<T, PairMessageError>;

/** The procedural cons: the parts live in the closure; no data structure is built. */
export const consProc =
  <T>(x: T, y: T): ProcPair<T> =>
  (m) =>
    m === 0 ? ok(x) : m === 1 ? ok(y) : err({ _tag: "UnknownMessage", m });

/** Applies the dispatch with message 0: the book's (car z) = (z 0). */
export const carProc = <T>(z: ProcPair<T>): Result<T, PairMessageError> => z(0);

/** Applies the dispatch with message 1: the book's (cdr z) = (z 1). */
export const cdrProc = <T>(z: ProcPair<T>): Result<T, PairMessageError> => z(1);

// Extended exercise: interval arithmetic (2.1.4).

/** A closed interval [lo, hi] holding the range of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** The interval package's domain errors. */
export type IntervalError =
  | { readonly _tag: "UnorderedBounds"; readonly a: number; readonly b: number }
  | { readonly _tag: "ZeroSpan"; readonly lo: number; readonly hi: number };

/**
 * Builds an interval from its two bounds, enforcing lo <= hi: this edition's
 * answer to the invariant question exercise 2.9 raises, settled once here
 * rather than left open at every call site.
 */
export const makeInterval = (a: number, b: number): Result<Interval, IntervalError> =>
  a <= b ? ok({ lo: a, hi: b }) : err({ _tag: "UnorderedBounds", a, b });

/** Adds the bounds pairwise; ordered bounds stay ordered. */
export const addInterval = (x: Interval, y: Interval): Interval => ({
  lo: x.lo + y.lo,
  hi: x.hi + y.hi,
});

/** Multiplies through the four corner products and keeps the extreme ones. */
export const mulInterval = (x: Interval, y: Interval): Interval => {
  const p1 = x.lo * y.lo;
  const p2 = x.lo * y.hi;
  const p3 = x.hi * y.lo;
  const p4 = x.hi * y.hi;
  return { lo: Math.min(p1, p2, p3, p4), hi: Math.max(p1, p2, p3, p4) };
};

/**
 * Multiplies by the reciprocal of y, whose bounds are the reciprocals of
 * y's bounds in reverse order. This is the section's pre-exercise-2.10
 * spelling: it builds the reciprocal with the literal form, bypassing
 * makeInterval's ordered-bounds check on purpose, and says nothing about a
 * divisor that spans zero.
 */
export const divInterval = (x: Interval, y: Interval): Interval =>
  mulInterval(x, { lo: 1 / y.hi, hi: 1 / y.lo });

/** The alternate constructor: a center value and an additive tolerance. */
export const makeCenterWidth = (c: number, w: number): Interval => ({ lo: c - w, hi: c + w });

/** The midpoint of the two bounds: the book's center. */
export const center = (i: Interval): number => (i.lo + i.hi) / 2;

/** Half the difference of the two bounds: the book's width. */
export const width = (i: Interval): number => (i.hi - i.lo) / 2;

/** The parallel-resistance formula written the direct way: Lem's par1. */
export const par1 = (r1: Interval, r2: Interval): Interval =>
  divInterval(mulInterval(r1, r2), addInterval(r1, r2));

/** The same formula with each uncertain quantity mentioned once: Lem's par2. */
export const par2 = (r1: Interval, r2: Interval): Interval => {
  const one: Interval = { lo: 1, hi: 1 };
  return divInterval(one, addInterval(divInterval(one, r1), divInterval(one, r2)));
};
