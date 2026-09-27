// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.4: our model of evaluation allows combinations whose
 * operators are compound expressions. TypeScript's operators are syntax
 * rather than values, so a conditional cannot return an operator name;
 * it can return a procedure, and the call applies whichever one the
 * conditional picked. Describe the behavior of this procedure.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.4 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function aPlusAbsB(_a: number, _b: number): number {
  throw new PendingSolution();
}
