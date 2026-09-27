// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Process } from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.38: Peter deposits 10, Paul withdraws 20, and Mary
 * withdraws half the balance, all on a joint account that starts at
 * 100. Each command is the book's three-step `set!`: access the
 * balance, compute the new value, set it. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_38.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.38 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The joint account: one balance cell the three commands mutate. */
export interface JointAccount {
  balance: number;
}

/** Makes the account with the exercise's initial 100. */
export function makeJointAccount(): JointAccount {
  throw new PendingSolution();
}

/** Peter's `(set! balance (+ balance 10))` as three steps. */
export function peterProcess(_account: JointAccount): Process {
  throw new PendingSolution();
}

/** Paul's `(set! balance (- balance 20))` as three steps. */
export function paulProcess(_account: JointAccount): Process {
  throw new PendingSolution();
}

/** Mary's `(set! balance (- balance (/ balance 2)))` as three steps:
 * the two accesses of the book's expression are two separate steps, so
 * an interleaving can change the balance between them. */
export function maryProcess(_account: JointAccount): Process {
  throw new PendingSolution();
}

/** Part 1: the values after the three transactions complete in every
 * sequential order (the six permutations of the three processes), in
 * the order the permutations run. */
export function sequentialValues(): ReadonlyArray<number> {
  throw new PendingSolution();
}

/** Part 2: every distinct value over all interleavings of the three
 * three-step commands, ascending. */
export function interleavedValues(): ReadonlyArray<number> {
  throw new PendingSolution();
}

/** Part 2's timing-diagram answer: the values only interleaving can
 * produce, those absent from every sequential order, ascending. */
export function interleavingOnlyValues(): ReadonlyArray<number> {
  throw new PendingSolution();
}
