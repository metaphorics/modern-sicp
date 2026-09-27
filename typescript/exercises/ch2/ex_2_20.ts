// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.20: same parity, first argument plus any number more. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.20 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The first argument followed by the rest arguments of its parity. */
export function sameParity(_first: number, ..._rest: ReadonlyArray<number>): List<number> {
  throw new PendingSolution();
}
