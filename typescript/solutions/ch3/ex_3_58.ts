// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { consStream, type Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.58: the book's `expand` is long division. Each step
 * emits one radix digit: the quotient of `num * radix` over `den` is
 * the next digit of the expansion of `num/den` in that radix, and the
 * remainder restarts the step with the same `den` and `radix`. So the
 * stream is the successive digits of `num/den` written in the radix,
 * one digit per element, trailing in zeros when the division comes
 * out even and repeating otherwise.
 */

/** The book's `expand`: the successive digits of num/den in the
 * radix, one per element, by repeated long division. The book's
 * `quotient` is the integer quotient, `Math.trunc` here, and its
 * `remainder` is `%`, exact for the positive arguments the digit
 * recurrence produces. */
export const expand = (num: number, den: number, radix: number): Stream<number> =>
  consStream(Math.trunc((num * radix) / den), () => expand((num * radix) % den, den, radix));
