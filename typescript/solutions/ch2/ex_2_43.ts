// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import {
  enumerateInterval,
  filter,
  flatmap,
  list,
  map,
} from "../../packages/ch2/src/02-picture-language.js";
import { adjoinPosition, emptyBoard, isSafe } from "./ex_2_42.js";

/**
 * Exercise 2.43: Louis Reasoner's eight-queens, with the two nested
 * mappings interchanged - rows outermost, the recursive call inside the
 * per-row map. The candidate placements are the same set of row-plus-rest
 * combinations in a different order, so the isSafe filter keeps exactly
 * the same answers; the cost, not the correctness, is what changes.
 */

/** Louis's queens: the (k-1)-solutions are recomputed for every row. */
export const queensSwapped = (boardSize: number): List<List<number>> => {
  const queenCols = (k: number): List<List<number>> =>
    k === 0
      ? list(emptyBoard())
      : filter(
          (positions) => isSafe(k, positions),
          flatmap(
            (newRow) => map((rest) => adjoinPosition(newRow, k, rest), queenCols(k - 1)),
            enumerateInterval(1, boardSize),
          ),
        );
  return queenCols(boardSize);
};
