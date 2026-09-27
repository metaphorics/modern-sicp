// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import {
  cons,
  enumerateInterval,
  filter,
  flatmap,
  list,
  map,
  nil,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.42: the eight-queens puzzle by nested mappings. A partial
 * solution is the list of row indices for the placed columns, most
 * recent (k-th) column at the head. Placing a queen conses a row in
 * 1..boardSize onto each (k-1)-solution, and a placement is safe when
 * its row hits no earlier row in the same row or either diagonal.
 */

/** The board with no queens placed. */
export const emptyBoard = (): List<number> => nil;

/** Places a queen in row `row` of column `k`, before the earlier columns. */
export const adjoinPosition = (row: number, _k: number, positions: List<number>): List<number> =>
  cons(row, positions);

/** Whether the newest queen (column `k`) attacks none of the earlier ones. */
export const isSafe = (k: number, positions: List<number>): boolean => {
  if (k <= 1 || positions._tag === "Nil") {
    return true;
  }
  const row = positions.head;
  const check = (rest: List<number>, col: number): boolean =>
    rest._tag === "Nil" ||
    col >= k ||
    (rest.head !== row && Math.abs(rest.head - row) !== col && check(rest.tail, col + 1));
  return check(positions.tail, 1);
};

/** One solution per list: the row choices for a boardSize-by-boardSize board. */
export const queens = (boardSize: number): List<List<number>> => {
  const queenCols = (k: number): List<List<number>> =>
    k === 0
      ? list(emptyBoard())
      : filter(
          (positions) => isSafe(k, positions),
          flatmap(
            (rest) =>
              map((newRow) => adjoinPosition(newRow, k, rest), enumerateInterval(1, boardSize)),
            queenCols(k - 1),
          ),
        );
  return queenCols(boardSize);
};
