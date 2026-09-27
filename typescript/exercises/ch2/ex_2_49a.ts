// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.49a: painter output as SVG strings. The wave and rogers
 * primitives render to exact SVG documents under the identity frame and
 * one transformed frame; the pins are byte comparisons.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.49a is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Renders the wave painter's SVG document under the identity frame. */
export function waveIdentitySvg(): string {
  throw new PendingSolution();
}
