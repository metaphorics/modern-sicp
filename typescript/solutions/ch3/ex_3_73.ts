// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { integralDelayed, type Stream, streamMap2 } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.73: the RC circuit of figure 3.33 as a signal processor.
 * The statement's formula reads v = v0 + (1/C) INT of i dt + R i, and
 * the signal-flow diagram draws the current feeding two paths, a
 * resistive one scaled by R and a capacitive one scaled by 1/C then
 * integrated, which meet in an adder. The same circuit law is the
 * differential equation C dv/dt = i - v/R: the resistor and capacitor
 * share the injected current, so the capacitor voltage integrates the
 * surplus (i - v/R)/C. Spelled as the feedback loop the diagram
 * implies,
 *
 *   v = integral(delayed(() => (i - v/R)/C), v0, dt)
 *
 * the integrand consumes the very voltage stream being defined, so the
 * integrand must arrive delayed exactly as in 3.5.4. The module's
 * `integralDelayed` is the section's own integral with a delayed
 * integrand: it emits the initial value at once and, each time a tail
 * is demanded, adds dt times the forced integrand to the integral so
 * far (the book's Euler rectangle).
 */

/** The statement's `rc`: takes the resistance R, the capacitance C, and
 * the time step dt, and answers a procedure from the current stream
 * and the initial capacitor voltage v0 to the voltage stream. The
 * integrand (i - v/R)/C is the capacitor current over C; it reads the
 * `voltage` being defined, so the whole integrand is delayed until
 * `integralDelayed` asks for the tail. */
export const rc = (
  R: number,
  C: number,
  dt: number,
): ((current: Stream<number>, v0: number) => Stream<number>) => {
  const voltageFor = (current: Stream<number>, v0: number): Stream<number> => {
    const voltage: Stream<number> = integralDelayed(
      () => streamMap2((i, v) => (i - v / R) / C, current, voltage),
      v0,
      dt,
    );
    return voltage;
  };
  return voltageFor;
};

/** The statement's example: R = 5 ohms, C = 1 farad, dt = 0.5 s. */
export const rc1 = rc(5, 1, 0.5);
