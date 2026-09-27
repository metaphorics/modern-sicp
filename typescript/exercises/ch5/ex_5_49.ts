// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.49: read-compile-execute loop machine. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.49 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_49(): never {
  throw new PendingSolution();
}
