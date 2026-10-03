// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  consStream,
  type Stream,
  streamCdr,
  streamMap,
  theEmptyStream,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.77: the integers-starting-from-style `integral`, made
 * safe for feedback systems. The statement's recursive version
 * advances the initial value by dt times the integrand's car and
 * recurses on the integrand's cdr:
 *
 *   const integral = (integrand, initialValue, dt) =>
 *     consStream(initialValue, () =>
 *       integrand === null
 *         ? null
 *         : integral(streamCdr(integrand), dt * integrand.head + initialValue, dt));
 *
 * In a system with loops, such as the `solve` procedure of 3.5.4, the
 * integrand is the mapping of f over the very stream being defined, so
 * this version needs the answer's first element before the answer
 * exists. The exercise modifies it to expect the integrand as a
 * delayed argument: the head is still produced without the integrand,
 * and the force happens inside the delayed tail, so nothing of the
 * integrand is evaluated until a caller demands more than the initial
 * value.
 */

/** The book's modified `integral`: the integrand arrives delayed and
 * is forced only inside the tail promise, once per demanded element.
 * The recursion re-delays the integrand's cdr, and a null integrand
 * ends the stream, the statement's empty-stream guard. Restated
 * after the exercise; the module's `integralDelayed` is the section's
 * own addStreams spelling of the same contract. */
export const delayedIntegral = (
  delayedIntegrand: () => Stream<number>,
  initialValue: number,
  dt: number,
): Stream<number> =>
  consStream(initialValue, () => {
    const integrand = delayedIntegrand();
    if (integrand === null) {
      return theEmptyStream;
    }
    return delayedIntegral(() => streamCdr(integrand), initialValue + dt * integrand.head, dt);
  });

/** The book's `solve` of 3.5.4, now constructible: dy/dt = f(y) from
 * y0 at step dt, with the feedback integrand `streamMap(f, y)` delayed
 * until the integral's tails ask for it. The binding `y` is closed
 * over by the promise and exists by the time any tail is forced. */
export const solve = (f: (y: number) => number, y0: number, dt: number): Stream<number> => {
  const y: Stream<number> = delayedIntegral(() => streamMap(f, y), y0, dt);
  return y;
};

/** The statement's unmodified recursive `integral`, restated for the
 * demonstration: the integrand arrives as a stream, not a delay, so a
 * feedback loop must produce the integrand's first element before the
 * integral it reads exists. */
export const plainRecursiveIntegral = (
  integrand: Stream<number>,
  initialValue: number,
  dt: number,
): Stream<number> => {
  if (integrand === null) {
    return theEmptyStream;
  }
  return consStream(initialValue, () =>
    plainRecursiveIntegral(streamCdr(integrand), initialValue + dt * integrand.head, dt),
  );
};

/** The statement's `solve` attempted over the unmodified integral:
 * the plain integral needs its integrand as an already-evaluated
 * stream, so building the loop must evaluate `streamMap(f, y)` right
 * now, while the binding `y` is still initializing. TypeScript
 * rejects that read at run time with a temporal-dead-zone
 * ReferenceError instead of answering a stream; the delayed version
 * above is what makes the definition legal. */
export const solveWithPlainIntegral = (
  f: (y: number) => number,
  y0: number,
  dt: number,
): Stream<number> => {
  const integrand = (): Stream<number> => streamMap(f, y);
  const y: Stream<number> = plainRecursiveIntegral(integrand(), y0, dt);
  return y;
};
