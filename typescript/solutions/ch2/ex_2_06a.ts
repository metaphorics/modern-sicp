// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.6a (added by this edition, extends exercise 2.6): arithmetic
 * on three plus the successor function, checked through the
 * encoded-decoding property. three applies its step three times; succ is
 * the book's add1 under its arithmetic name; decoding a numeral applies
 * it to the successor step and a zero seed, so decode(succ(n)) =
 * decode(n) + 1 and decode(churchAdd(m, n)) = decode(m) + decode(n) are
 * the property the assertions pin.
 */
import type { Church } from "./ex_2_06.js";

/** Three, defined directly. */
export const three: Church = (f) => (x) => f(f(f(x)));

/** The successor function: the operation of adding 1 at the Church level. */
export const succ =
  (n: Church): Church =>
  (f) =>
  (x) =>
    f(n(f)(x));

/** Decodes a numeral: applies it to the successor step and a zero seed. */
export const churchToInt = (n: Church): number => n((x: number) => x + 1)(0);
