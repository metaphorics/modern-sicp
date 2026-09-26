// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.12: the environment interface has three procedures that all
 * walk the frame chain looking for one name. The demand: factor that walk
 * out once, as abstract traversals (one over a frame's bindings, one over
 * the environment chain), and define lookup-variable-value,
 * set-variable-value!, and define-variable! in terms of them.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.12 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A three-frame chain exercises every traversal path: inner, middle,
 * outer, and a name that lives in no frame at all. */
export const threeFrameProgram = [
  "(define a 1)",
  "((lambda () (define b 2) ((lambda () (define c 3) (list a b c)))))",
];

export function ex_4_12(): string {
  throw new PendingSolution();
}
