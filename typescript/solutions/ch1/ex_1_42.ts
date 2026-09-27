// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.42: function composition.
 *
 * compose(f, g) is the function x |-> f(g(x)) - "f after g", the argument
 * closest to the data applied first.
 */
export function compose(f: (x: number) => number, g: (x: number) => number): (x: number) => number {
  return (x: number): number => f(g(x));
}
