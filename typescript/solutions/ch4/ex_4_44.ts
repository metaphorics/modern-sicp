// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.44: queens under the search experiment. The board is a
 * list of rows, built column by column: each column draws its row with
 * a single choice over the board's rows and requires the placement
 * safe against every earlier column (same row, or diagonal distance).
 * The nondeterminism is one choice per column; everything else is
 * ordinary recursion. The answer lists rows with the newest column
 * first, the book's cons-built order, so the pins read the same as the
 * sibling editions'.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

/** The queens program for one board size. */
export const queensSource = (size: number): string => `
const anIntegerBetween = (low: number, high: number): number => {
  require(low <= high);
  return choose(low, anIntegerBetween(low + 1, high));
};
const safe = (placed: number[], row: number): boolean => {
  let ok = true;
  let i = 0;
  while (i < placed.length) {
    const item = placed[i];
    const other = item === undefined ? 0 : item;
    const distance = placed.length - i;
    if (other === row || other - row === distance || row - other === distance) {
      ok = false;
    }
    i = i + 1;
  }
  return ok;
};
const queens = (placed: number[], n: number): number[] => {
  if (placed.length === n) {
    return placed;
  }
  const row = anIntegerBetween(1, n);
  require(safe(placed, row));
  return queens([...placed, row], n);
};
const board = queens([], ${size});
[...board].reverse();
`;

/** Every solution of one board size, in search order. */
export const solutions = (size: number): ReadonlyArray<string> =>
  runAmbAnswers(queensSource(size), "amb-depth-first-experiment", 1).answers.map((value) =>
    format(value),
  );

export function ex_4_44(): string {
  return (
    "One choice per column, everything else ordinary recursion: the 8x8 board answers 92 " +
    "solutions, the first [4, 2, 7, 3, 6, 8, 5, 1]; the 4x4 board answers [3, 1, 4, 2] " +
    "then [2, 4, 1, 3]; the 6x6 board answers 4 solutions beginning [5, 3, 1, 6, 4, 2]. " +
    "The counts and first boards match the sibling editions' pins."
  );
}
