// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.35: reverse-engineer source from Figure 5.18. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.35 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_35(): never {
  throw new PendingSolution();
}
