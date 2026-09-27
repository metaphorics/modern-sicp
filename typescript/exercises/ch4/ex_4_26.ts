// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.26: Ben and Alyssa disagree over implementing unless as a special
 * form versus keeping it a procedure. Fill in both sides: implement unless as
 * a derived expression, and give an example where having unless as a
 * procedure (usable with higher-order procedures) matters.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.26 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_26(): string {
  throw new PendingSolution();
}
