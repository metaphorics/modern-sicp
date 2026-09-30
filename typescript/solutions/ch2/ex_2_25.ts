// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.25: pick out the 7. Each car/cdr pair becomes one index:
 * `car` is [0] and each `cdr` is [1]. The nested combinations are
 * readonly-array tuples, annotated level by level so the chains
 * typecheck.
 */

/** The 7 inside [1, 3, [5, 7], 9] sits at [2][1]. */
export function sevenFromFirst(): number {
  const first: readonly [number, number, readonly [number, number], number] = [1, 3, [5, 7], 9];
  return first[2][1];
}

/** The 7 inside [[7]] sits at [0][0]. */
export function sevenFromSecond(): number {
  const second: readonly [readonly [number]] = [[7]];
  return second[0][0];
}

// The nesting of [1, [2, [3, [4, [5, [6, 7]]]]]], one level per alias.
type Level6 = readonly [number, number];
type Level5 = readonly [number, Level6];
type Level4 = readonly [number, Level5];
type Level3 = readonly [number, Level4];
type Level2 = readonly [number, Level3];
type Level1 = readonly [number, Level2];

/** The 7 in [1, [2, [3, [4, [5, [6, 7]]]]]] is six levels down: [1] six times. */
export function sevenFromThird(): number {
  const third: Level1 = [1, [2, [3, [4, [5, [6, 7]]]]]];
  return third[1][1][1][1][1][1];
}
