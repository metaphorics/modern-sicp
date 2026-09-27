// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.16: instruction tracing on and off. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.16 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_16(): never {
  throw new PendingSolution();
}
