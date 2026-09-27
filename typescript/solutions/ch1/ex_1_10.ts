// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.10: Ackermann's function.
 *
 * The values asked for are ackermann(1, 10), ackermann(2, 4), and
 * ackermann(3, 3). The wrappers make the growth pattern readable:
 * ackermannF is 2n, ackermannG doubles, ackermannH iterates the
 * doubling (a tower of twos), ackermannK is the 5n^2 comparison the
 * statement spells out.
 */
export const ackermann = (x: number, y: number): number => {
  if (y === 0) {
    return 0;
  }
  if (x === 0) {
    return 2 * y;
  }
  if (y === 1) {
    return 2;
  }
  return ackermann(x - 1, ackermann(x, y - 1));
};

export const ackermannF = (n: number): number => ackermann(0, n);

export const ackermannG = (n: number): number => ackermann(1, n);

export const ackermannH = (n: number): number => ackermann(2, n);

export const ackermannK = (n: number): number => 5 * n * n;
