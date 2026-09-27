// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.42: lexical addressing in code generators. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.42 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_42(): never {
  throw new PendingSolution();
}
