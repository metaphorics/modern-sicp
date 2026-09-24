// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** The parity classifier the section's table test drives. */
export const parity = (n: number): string => (n % 2 === 0 ? "even" : "odd");

/** The distance between two integers, symmetric by construction. */
export const absDiff = (a: number, b: number): number => Math.abs(a - b);
