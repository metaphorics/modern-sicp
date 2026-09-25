// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.1: the sequence of the statement, evaluated in order.
 *
 * Each expression contributes the value it produces; the two `const`
 * declarations are statements and produce nothing of their own. The
 * `a === b` response is the run's only non-numeric value. The values are
 * the results a script prints, one per line, in the order they appear.
 */
export function ex_1_01(): readonly (number | boolean)[] {
  const a: number = 3;
  const b: number = a + 1;
  return [
    10,
    5 + 3 + 4,
    9 - 1,
    6 / 2,
    2 * 4 + (4 - 6),
    a + b + a * b,
    a === b,
    b > a && b < a * b ? b : a,
    a === 4 ? 6 : b === 4 ? 6 + 7 + a : 25,
    2 + (b > a ? b : a),
    (a > b ? a : a < b ? b : -1) * (a + 1),
  ];
}
