// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { TermList } from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.97a (added by this edition, extends exercise 2.97): the
 * drop gcd-terms after reduce check. A reduced numerator and denominator
 * share only a unit: gcd-terms on the reduced pair answers a degree-zero
 * constant, reducing the reduced pair answers the same pair, and the
 * 2.95 products Q1 over Q2 reduce to exactly P2 over P3. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch2/ex_2_97a.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.97a is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Runs gcd-terms on a pair already reduced by reduce-terms. */
export function gcdAfterReduce(_n: TermList, _d: TermList): TermList {
  throw new PendingSolution();
}
