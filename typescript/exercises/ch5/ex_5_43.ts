// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.43: scan out internal definitions. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.43 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_43(): never {
  throw new PendingSolution();
}
