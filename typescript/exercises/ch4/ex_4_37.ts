// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.37: Ben Bitdiddle's claim that computing ksq first and requiring (>= hsq ksq) prunes the triple search more efficiently than exercise 4.35's.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.37 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_37(): string {
  throw new PendingSolution();
}
