// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.77: integral with a delayed integrand. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_77.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.77 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's modified `integral`: the integrand arrives delayed and
 * is forced only inside the tail promise. */
export function delayedIntegral(
  _delayedIntegrand: () => Stream<number>,
  _initialValue: number,
  _dt: number,
): Stream<number> {
  throw new PendingSolution();
}

/** The book's `solve`: dy/dt = f(y) from y0 at step dt, built over
 * the delayed integral. */
export function solve(_f: (y: number) => number, _y0: number, _dt: number): Stream<number> {
  throw new PendingSolution();
}

/** The statement's unmodified recursive `integral`: the integrand
 * arrives as a stream, not a delay. */
export function plainRecursiveIntegral(
  _integrand: Stream<number>,
  _initialValue: number,
  _dt: number,
): Stream<number> {
  throw new PendingSolution();
}

/** The statement's `solve` attempted over the unmodified integral:
 * constructing the loop must read `y` while it is still
 * initializing. */
export function solveWithPlainIntegral(
  _f: (y: number) => number,
  _y0: number,
  _dt: number,
): Stream<number> {
  throw new PendingSolution();
}
