// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Painter } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.51: below, the vertical mirror of beside, two ways. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.51 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** below that splits the frame with two transform-painter calls. */
export function belowDirect(_painter1: Painter, _painter2: Painter): Painter {
  throw new PendingSolution();
}

/** below built from beside and the quarter-turn rotations. */
export function belowViaRotation(_painter1: Painter, _painter2: Painter): Painter {
  throw new PendingSolution();
}
