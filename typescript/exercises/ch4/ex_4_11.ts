// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.11: the book represents a frame as a pair of parallel lists;
 * here it asks for the other classic shape: a frame is a list of bindings,
 * one (name value) entry each, and the environment operations are
 * rewritten over that association list. The pending part is the frame
 * representation and new lookup-variable-value, set-variable-value!, and
 * define-variable! procedures that walk it.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.11 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A frame as the book now draws it: a list of (name value) bindings. */
export const sampleFrameText = "((a 1) (b 2))";

/** Programs the new operations must serve: define, lookup, set!, and the
 * shared-frame visibility of set! through a nested frame. */
export const assocPrograms = [
  "(define a 1)",
  "((lambda () (define b 2) b))",
  "((lambda () (define b 2) (set! a 10) a))",
];

export function ex_4_11(): string {
  throw new PendingSolution();
}
