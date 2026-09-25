// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type Connector, multiplier } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.34: Louis Reasoner's squarer, the book's
 * `(multiplier a a b)`. The flaw the exercise asks for: the
 * multiplier box sees two factor terminals that always agree, so a
 * value on `b` leaves it no second fact to divide by. Setting `a`
 * propagates (`b = a * a`), but setting `b` alone determines neither
 * factor, and the multiplier's process-new-value falls through every
 * branch: `a` stays unset, no contradiction is ever raised, and the
 * network is silently stuck. The pins demonstrate both directions.
 */

/** Wires Louis's device: `b` is constrained to `a * a` by one
 * multiplier whose two factor terminals are the same connector. */
export const squarerFromMultiplier = (a: Connector, b: Connector): void => {
  multiplier(a, a, b);
};
