// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.10: division checked against a divisor that spans zero.
 * The reciprocal of an interval with `lo <= 0 <= hi` is not an interval -
 * it runs through infinity - so the checked procedure reports the
 * condition as a typed error value instead of returning a number. The
 * test includes the boundary cases: an interval that only touches zero
 * at a bound has no reciprocal either.
 */
type Interval = { readonly lo: number; readonly hi: number };

/** The failure of a division whose divisor spans or touches zero. */
export type DivError = { readonly _tag: "ZeroSpan"; readonly lo: number; readonly hi: number };

/** The book's two-sided result: a value or an error, never a throw. */
export type Result<A, E> =
  | { readonly _tag: "Ok"; readonly value: A }
  | { readonly _tag: "Error"; readonly error: E };

const ok = <A>(value: A): Result<A, never> => ({ _tag: "Ok", value });

const err = <E>(error: E): Result<never, E> => ({ _tag: "Error", error });

const mulInterval = (x: Interval, y: Interval): Interval => {
  const p1 = x.lo * y.lo;
  const p2 = x.lo * y.hi;
  const p3 = x.hi * y.lo;
  const p4 = x.hi * y.hi;
  return { lo: Math.min(p1, p2, p3, p4), hi: Math.max(p1, p2, p3, p4) };
};

/** Divides x by y, reporting ZeroSpan when y spans or touches zero. */
export const divIntervalChecked = (x: Interval, y: Interval): Result<Interval, DivError> => {
  if (y.lo <= 0 && y.hi >= 0) {
    return err({ _tag: "ZeroSpan", lo: y.lo, hi: y.hi });
  }
  return ok(mulInterval(x, { lo: 1 / y.hi, hi: 1 / y.lo }));
};
