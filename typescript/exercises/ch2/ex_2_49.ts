// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Painter } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.49: primitive painters from segment lists. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.49 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** a. The outline of the frame. */
export function outline(): Painter {
  throw new PendingSolution();
}

/** b. The diagonals of the frame, drawn as an X. */
export function crossPainter(): Painter {
  throw new PendingSolution();
}

/** c. The diamond connecting the midpoints of the frame's sides. */
export function diamond(): Painter {
  throw new PendingSolution();
}

/** d. The wave painter, rebuilt from the section's wave segment list. */
export function waveFromSegments(): Painter {
  throw new PendingSolution();
}
