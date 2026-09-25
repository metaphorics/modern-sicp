// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.11: f(n) = n for n < 3, f(n) = f(n-1) + 2f(n-2) + 3f(n-3)
 * for n >= 3.
 *
 * The recursive spelling is the definition read aloud. The iterative
 * spelling keeps the last three values as state variables and slides
 * the window, the factorialIter shape: an explicit loop, since a
 * tail-recursive call chain buys nothing on Node.
 */
export const fRecursive = (n: number): number =>
  n < 3 ? n : fRecursive(n - 1) + 2 * fRecursive(n - 2) + 3 * fRecursive(n - 3);

export const fIter = (n: number): number => {
  if (n < 3) {
    return n;
  }
  let a = 0; // f(k - 2)
  let b = 1; // f(k - 1)
  let c = 2; // f(k), starting at k = 2
  let k = 2;
  while (k < n) {
    const next = c + 2 * b + 3 * a;
    a = b;
    b = c;
    c = next;
    k += 1;
  }
  return c;
};
