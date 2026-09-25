// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import { accumulate } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.34: Horner's rule for evaluating a polynomial. The rule
 * rewrites a0 + a1*x + ... + an*x^n as
 * a0 + x*(a1 + x*(a2 + ... + x*an)), a right fold over the coefficients
 * ordered from the constant term up: each step multiplies the value of
 * the higher terms by x and adds the next coefficient.
 */

/** Evaluates the polynomial with coefficients `coefficientSequence` at `x`. */
export const hornerEval = (x: number, coefficientSequence: List<number>): number =>
  accumulate((thisCoeff, higherTerms) => thisCoeff + x * higherTerms, 0, coefficientSequence);
