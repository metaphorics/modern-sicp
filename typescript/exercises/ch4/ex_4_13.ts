// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.13: the language has make-unbound! nowhere; the demand is to
 * add it. Give make-unbound! a precise specification (which frame loses
 * the binding, what happens when no frame has it), implement it in the
 * evaluator, and show the shadowing behavior: after unbinding an inner
 * binding the outer one is visible again, and unbinding a name that is
 * bound nowhere is an error.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.13 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The new form: x's binding disappears from exactly one frame. */
export const unbindForm = "(make-unbound! x)";

/** Shadowing scenario: an inner define hides the outer a, an unbind
 * restores it, and an unbind of a missing name must fail. */
export const shadowPrograms = [
  "(define a 1)",
  "((lambda () (define a 2) a))",
  "((lambda () (define a 2) (make-unbound! a) a))",
];

export function ex_4_13(): string {
  throw new PendingSolution();
}
