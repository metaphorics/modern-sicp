// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  integralDelayed,
  type Stream,
  streamMap,
  streamMap2,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.80: the series RLC circuit. The statement combines the
 * component laws `vR = iR R`, `vL = L diL/dt`, `iC = C dvC/dt` with
 * the connection laws `iR = iL = -iC` and `vC = vL + vR` into a pair
 * of differential equations for the state of the circuit:
 *
 *     dvC/dt = -iL / C
 *     diL/dt = (1/L) vC - (R/L) iL
 *
 * (the statement's signs: the capacitor voltage decreases when the
 * capacitor drives a positive current out of its positive terminal,
 * the resistor dissipates, and the inductor picks up `vC - vR`).
 * Figure 3.37 is two `integral` boxes: the `vC` loop's integrand is
 * the `iL` signal scaled by `-1/C`, and the `iL` loop's integrand is
 * the sum of `vC` scaled by `1/L` and `iL` scaled by `-R/L`.
 */

/** The pair the statement's `RLC` answers: the streams of the state
 * variables, in the statement's order (`vC` first, then `iL`). */
export interface RlcStreams {
  readonly vC: Stream<number>;
  readonly iL: Stream<number>;
}

/** The book's `RLC`: takes the circuit parameters and the time step
 * and answers a procedure from the initial values `vC0` and `iL0` to
 * the pair of state streams, the two integrators wired in a loop
 * exactly as Figure 3.37 draws them. */
export const RLC =
  (R: number, L: number, C: number, dt: number): ((vC0: number, iL0: number) => RlcStreams) =>
  (vC0, iL0) => {
    const vC: Stream<number> = integralDelayed(() => streamMap((i) => -i / C, iL), vC0, dt);
    const iL: Stream<number> = integralDelayed(
      () => streamMap2((v, i) => v / L - (R / L) * i, vC, iL),
      iL0,
      dt,
    );
    return { vC, iL };
  };
