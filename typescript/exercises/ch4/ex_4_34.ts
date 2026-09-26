// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.34: modify the driver loop so lazy pairs and lists print in
 * some reasonable way, answering what to do about infinite lists; the
 * representation of lazy pairs may need to change so the evaluator can
 * identify them.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.34 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_34(): string {
  throw new PendingSolution();
}
