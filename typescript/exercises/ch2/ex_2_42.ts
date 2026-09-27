// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.42: the eight-queens puzzle by nested mappings. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.42 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The board with no queens placed. */
export function emptyBoard(): List<number> {
  throw new PendingSolution();
}

/** Places a queen in row `row` of column `k`, before the earlier columns. */
export function adjoinPosition(_row: number, _k: number, _positions: List<number>): List<number> {
  throw new PendingSolution();
}

/** Whether the newest queen (column `k`) attacks none of the earlier ones. */
export function isSafe(_k: number, _positions: List<number>): boolean {
  throw new PendingSolution();
}

/** One solution per list: the row choices for a boardSize-by-boardSize board. */
export function queens(_boardSize: number): List<List<number>> {
  throw new PendingSolution();
}
