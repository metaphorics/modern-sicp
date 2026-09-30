// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.72: interleave versus append. Fair combination keeps
 * every productive branch answering: the son query's rule-derived
 * Lamech sons appear among the asserted sons instead of after all
 * of them, so no branch waits behind another's unbounded output.
 * Appending would list all eight assertions first and the two
 * derived sons last — fine on finite data, starvation behind an
 * infinite assertion stream. The pin reuses the delayed son order
 * and contrasts it with the appended one.
 */
import { sonOrders } from "./ex_4_71.js";

/** The interleaved and appended son orders side by side. */
export const interleaveVsAppend = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] =>
  sonOrders();

export function ex_4_72(): string {
  const [interleaved, appended] = interleaveVsAppend();
  return (
    `Interleave answers ${interleaved.length} sons with rule hits up ` +
    `front; append answers the same ${appended.length} with rules last. ` +
    `Fairness is productive coexistence: every branch answers without ` +
    `waiting behind another's unbounded output.`
  );
}
