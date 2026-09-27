// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.36: accumulate-n, collecting the nth elements of a list of lists. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.36 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Accumulates the columns of `seqs` as accumulate accumulates its rows. */
export function accumulateN<A, B>(_op: (x: A, y: B) => B, _init: B, _seqs: List<List<A>>): List<B> {
  throw new PendingSolution();
}
