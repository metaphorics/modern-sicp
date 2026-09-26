// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.3: rewrite eval so that the dispatch on special forms is done
 * in data-directed style: a table keyed by the operator symbol name, one
 * operation per form, installed with put and fetched with get, as in the
 * data-directed differentiation of exercise 2.73. A pair whose car is not
 * installed in the table is a procedure application.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.3 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_03(): string {
  throw new PendingSolution();
}
