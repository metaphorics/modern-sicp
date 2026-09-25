// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Painter } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.44: up-split, the mirror of right-split that branches upwards. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.44 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's up-split: the painter below two half-size copies side by side. */
export function upSplit(_painter: Painter, _n: number): Painter {
  throw new PendingSolution();
}
