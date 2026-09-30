// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 0.1: three TypeScript interaction sessions.
 *
 * Each session evaluates in order and contributes its numeric responses to
 * one list, which the test asserts. The equality comparison in session 2
 * answers `false`; the worked example shows it in the transcript rather
 * than in this numeric list.
 */
export function ex_0_01(): readonly number[] {
  const a: number = 3;
  const b: number = a + 1;
  const square = (x: number): number => x * x;
  return [
    486,
    100,
    5 + 3 + 4,
    10 - 9,
    2 * 4 + (4 - 6),
    a + b + a * b,
    b > a && b < a * b ? b : a,
    a === 4 ? 6 : b === 4 ? 6 + 7 + a : 25 + a,
    2 + (b > a ? b : a),
    (a > b ? a : b > a ? b : -1) * (a + 1),
    square(21),
    square(2 + 5),
    square(square(3)),
  ];
}
