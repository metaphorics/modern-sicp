// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.77: nested tagged values need layered dispatch: Louis Reasoner's
 * magnitude surprise: apply-generic strips one tag per dispatch, so the
 * complex-level selectors strip the outer tag and re-dispatch on the inner
 * rectangular/polar tag. Pending scaffold; the solution and its rationale live
 * in solutions/ch2/ex_2_77.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.77 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Placeholder: throws until the solution lands. */
export function scaffold(): never {
  throw new PendingSolution();
}
