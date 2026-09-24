// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 0.4: an exhaustive `area` over the three-variant `Shape` union,
 * then the compile experiment: delete the `Square` case, record the
 * compiler's message, restore, and say what exhaustiveness bought. The
 * statement lives in the section 0.4 chapter text.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 0.4 is not solved yet");
    this.name = "PendingSolution";
  }
}

export type Shape =
  | { readonly _tag: "Circle"; readonly r: number }
  | { readonly _tag: "Rect"; readonly w: number; readonly h: number }
  | { readonly _tag: "Square"; readonly s: number };

export function area(_s: Shape): number {
  throw new PendingSolution();
}
