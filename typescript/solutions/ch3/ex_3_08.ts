// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.8: operand evaluation order. The exercise is about the
 * host language's order for evaluating a procedure call's operands, so
 * this probe is deliberately the raw JavaScript closure the book asks
 * for (a captured `let` is the edition's sanctioned local state for
 * exactly this exercise): an Effect version would test Effect's own
 * sequencing, not the language's operand order. The probe multiplies
 * its argument into a remembered state and answers the product, so
 * whichever call runs first decides the answer the second call sees.
 */

/** Builds the probe procedure `f`: the first call sees state 1, every
 * later call sees the product of all earlier arguments. */
export const makeOrderProbe = (): ((x: number) => number) => {
  let state = 1;
  return (x: number) => {
    state = x * state;
    return state;
  };
};
