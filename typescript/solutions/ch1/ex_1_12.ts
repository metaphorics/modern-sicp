// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.12: Pascal's triangle by a recursive process.
 *
 * Each inside element is the sum of the two above it; the edges are 1.
 * row and col are 0-indexed, with 0 <= col <= row.
 */
export const pascal = (row: number, col: number): number => {
  if (col === 0 || col === row) {
    return 1;
  }
  return pascal(row - 1, col - 1) + pascal(row - 1, col);
};
