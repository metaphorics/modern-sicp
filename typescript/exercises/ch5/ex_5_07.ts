// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.07: test the designed machines on the simulator. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.07 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_07(): never {
  throw new PendingSolution();
}
