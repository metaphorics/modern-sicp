// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.13: the computational leg of the closed-form proof.
 *
 * The proof itself lives in ex_1_13.md (induction on n using
 * psi = (1 - sqrt(5)) / 2 and psi^2 = psi + 1). This cross-check is the
 * part a machine can hold: the closed form rounds to Fib(n) for every
 * n the floating-point evaluation can still resolve.
 */
const phi = (1 + Math.sqrt(5)) / 2;
const psi = (1 - Math.sqrt(5)) / 2;

/** The closed form (phi^n - psi^n) / sqrt(5), rounded to the nearest integer. */
export const closedFormFib = (n: number): number =>
  Math.round((phi ** n - psi ** n) / Math.sqrt(5));

/** The definition of section 1.2.2, iterated: the ground truth to round toward. */
export const fibDefinition = (n: number): number => {
  let a = 0;
  let b = 1;
  for (let k = 0; k < n; k += 1) {
    const next = a + b;
    a = b;
    b = next;
  }
  return a;
};
