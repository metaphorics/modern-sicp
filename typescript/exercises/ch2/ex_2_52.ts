// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Painter } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.52: changes at each level of the stratified design — added
 * smile segments on the primitive painter, a corner-split variant with
 * single up/right copies, and a square-limit variant that reorders the
 * corners.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.52 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The wave painter with an added smile. */
export function waveWithSmile(): Painter {
  throw new PendingSolution();
}
