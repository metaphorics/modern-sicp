// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.19: redo exercise 3.18 with an algorithm that takes only
 * a constant amount of space. The clever idea is Floyd's
 * tortoise-and-hare: two pointers race over the tails, the tortoise
 * advancing one tail at a time and the hare two. If the list is
 * cyclic the hare laps the tortoise inside the ring, so the two meet
 * at the same pair object; if the list is finite the hare reaches the
 * empty end first and the answer is false. The book's exercise-map
 * row shares the suffix-detection requirement across editions; the
 * space bound is the mechanism here: two locals, no visited set, no
 * per-pair memory.
 */

/** Whether the tail chase from `x` ever revisits a pair, decided in
 * constant space: the book's `has-cycle?` redone as tortoise and
 * hare. The tortoise takes one tail per step and the hare two, so on
 * a ring with a run of k pairs to the loop the hare closes the gap
 * one tail per step and the race ends in at most 2k steps. */
export const hasCycleConstantSpace = (x: MList<unknown>): boolean => {
  let tortoise: MList<unknown> = x;
  let hare: MList<unknown> = x;
  for (;;) {
    // On a finite list the hare, two tails per round, reaches the
    // empty end first.
    if (hare._tag === "MNil") {
      return false;
    }
    hare = hare.tail;
    if (hare._tag === "MNil") {
      return false;
    }
    hare = hare.tail;
    // The tortoise trails the hare, so it cannot be at the empty end
    // while the hare is not; the check narrows the tail step.
    if (tortoise._tag === "MNil") {
      return false;
    }
    tortoise = tortoise.tail;
    if (Object.is(hare, tortoise)) {
      return true;
    }
  }
};
