// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.1: the book asks for list-of-values written twice, once
 * evaluating the operands left to right and once right to left, because
 * Lisp leaves the order of operand evaluation unspecified. This edition
 * runs on a host that fixes the order: state which order JavaScript
 * evaluates call arguments in, write both book-faithful variants with
 * explicit recursion, and say which of them the module's listOfValues is.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.1 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_01(): string {
  throw new PendingSolution();
}
