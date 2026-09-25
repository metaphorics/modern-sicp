// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.34: Horner's rule for polynomial evaluation. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.34 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Evaluates the polynomial with coefficients `coefficientSequence` at `x`. */
export function hornerEval(_x: number, _coefficientSequence: List<number>): number {
  throw new PendingSolution();
}
