// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** The arithmetic session of the worked example, evaluated in order. */
export const arithmeticSession: readonly number[] = [486, 100, 5 + 3 + 4, 10 - 9, 2 * 4 + (4 - 6)];

/** The `square` session of the worked example, definition first. */
export const squareSession = (): readonly number[] => {
  const square = (x: number): number => x * x;
  return [square(21), square(2 + 5), square(square(3))];
};
