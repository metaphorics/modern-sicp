// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Painter } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.45: split, the pattern right-split and up-split share. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.45 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Returns the branching procedure the two combinators describe. */
export function split(
  _combiner: (a: Painter, b: Painter) => Painter,
  _subCombiner: (a: Painter, b: Painter) => Painter,
): (painter: Painter, n: number) => Painter {
  throw new PendingSolution();
}
