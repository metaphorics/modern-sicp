// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.3: Newton square-root machine. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.3 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_03(): never {
  throw new PendingSolution();
}
