// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.18: per-register tracing in make-register. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.18 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_18(): never {
  throw new PendingSolution();
}
