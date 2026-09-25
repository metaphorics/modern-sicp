// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.45: how many average damps does an n-th root need?
 *
 * The n-th root of x is the fixed point of y |-> x / y^(n-1). At the
 * fixed point y* that map has slope -(n-1), and each average damp
 * averages the slope toward 1, so d damps converge while
 * |2^d - n| < 2^d: one damp covers square and cube roots, two covers
 * 4th through 7th, three covers 8th through 15th. The search is capped
 * so an under-damped oscillation reports failure instead of running
 * forever - the once-damped 4th root is a persistent two-cycle whose
 * successive values never meet the tolerance (still unresolved after
 * 100000 steps in the probe behind this pin).
 */
const TOLERANCE = 0.00001;

const PROBE_X = 10;
const PROBE_STEP_CAP = 1000;

/** The damped fixed-point search for the n-th root of x, capped at stepCap steps. */
export function nthRoot(
  x: number,
  n: number,
  damps: number,
  stepCap = 1000,
): { root: number | null; steps: number } {
  let f = (y: number): number => x / y ** (n - 1);
  for (let i = 0; i < damps; i++) {
    const undamped = f;
    f = (y: number): number => (y + undamped(y)) / 2;
  }
  let guess = 1.0;
  for (let steps = 1; steps <= stepCap; steps++) {
    const next = f(guess);
    if (Math.abs(guess - next) < TOLERANCE) {
      return { root: next, steps };
    }
    guess = next;
  }
  return { root: null, steps: stepCap };
}

/** The smallest number of average damps that converges for the n-th root (n >= 2). */
export function dampsRequired(n: number): number {
  for (let damps = 1; ; damps++) {
    if (nthRoot(PROBE_X, n, damps, PROBE_STEP_CAP).root !== null) {
      return damps;
    }
  }
}
