// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Painter } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.50: flip-horiz, rotate-180 and rotate-270 as transforms. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.50 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's flip-horiz: mirror the painter left to right. */
export function flipHorizExercise(_painter: Painter): Painter {
  throw new PendingSolution();
}

/** The book's rotate-180: turn the painter upside down. */
export function rotate180Exercise(_painter: Painter): Painter {
  throw new PendingSolution();
}

/** The book's rotate-270: a quarter turn clockwise. */
export function rotate270Exercise(_painter: Painter): Painter {
  throw new PendingSolution();
}
