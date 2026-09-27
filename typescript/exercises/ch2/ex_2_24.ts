// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 2.24: the structure of (1 (2 (3 4))). */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.24 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** How (list 1 (list 2 (list 3 4))) prints. */
export function printedForm(): string {
  throw new PendingSolution();
}

/** The number of elements in the outer list. */
export function listLength(): number {
  throw new PendingSolution();
}

/** The number of leaves in the tree interpretation. */
export function leafCount(): number {
  throw new PendingSolution();
}
