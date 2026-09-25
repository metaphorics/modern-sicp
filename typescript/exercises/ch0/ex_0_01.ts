// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 0.1: translate the three Scheme sessions of section 0.2 into
 * TypeScript, producing the matching transcript lines.
 *
 * The scaffold returns the numeric results of the three sessions in order;
 * the `(a === b)` line appears in section 0.8's worked transcript instead.
 * The statement lives in the section 0.2 chapter text.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 0.1 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_0_01(): readonly number[] {
  throw new PendingSolution();
}
