// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type List, list } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.19: counting change over a list of denominations. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.19 is not solved yet");
    this.name = "PendingSolution";
  }
}

export const usCoins: List<number> = list(50, 25, 10, 5, 1);
export const ukCoins: List<number> = list(100, 50, 20, 10, 5, 2, 1, 0.5);

/** Is the denomination list used up? */
export function noMore(_coinValues: List<number>): boolean {
  throw new PendingSolution();
}

/** The value of the first denomination. */
export function firstDenomination(_coinValues: List<number>): number {
  throw new PendingSolution();
}

/** All denominations but the first. */
export function exceptFirstDenomination(_coinValues: List<number>): List<number> {
  throw new PendingSolution();
}

/** The number of ways to make `amount` from `coinValues`. */
export function cc(_amount: number, _coinValues: List<number>): number {
  throw new PendingSolution();
}
