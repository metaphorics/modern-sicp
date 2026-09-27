// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.17: environments under scanned-out internal definitions. The
 * statement asks for the frame structure at e3 under sequential and scanned
 * interpretation, the reason the transformed program has an extra frame, why
 * that difference can never change the behavior of a correct program, and a
 * design that implements simultaneous scope without constructing the extra
 * frame.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.17 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's procedure body: two internal defines and a final expression. */
export const bodySource = "(define u e1) (define v e2) e3";

/** The text's scanned-out shape: a let pre-binding u and v, then set!s. */
export const scannedSource =
  "(let ((u '*unassigned*) (v '*unassigned*)) (set! u e1) (set! v e2) e3)";

export function ex_4_17(): string {
  throw new PendingSolution();
}
