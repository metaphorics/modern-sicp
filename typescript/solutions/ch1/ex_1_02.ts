// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.2: the book's fraction as one nested application, every
 * operator written as a call.
 *
 * The nesting mirrors the expression tree: `div` at the root, the
 * numerator's `5 + 4 + ...` chain down the left, and the denominator's
 * `3(6 - 2)(2 - 7)` as a product of three factors. No infix operator
 * appears outside the four primitives.
 */
export const add = (x: number, y: number): number => x + y;

export const sub = (x: number, y: number): number => x - y;

export const mul = (x: number, y: number): number => x * y;

export const div = (x: number, y: number): number => x / y;

export function ex_1_02(): number {
  return div(add(5, add(4, sub(2, sub(3, add(6, div(4, 5)))))), mul(3, mul(sub(6, 2), sub(2, 7))));
}
