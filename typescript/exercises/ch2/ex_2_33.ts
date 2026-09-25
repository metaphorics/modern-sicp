// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.33: map, append, and length as accumulations. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.33 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's map, rebuilt as an accumulation. */
export function mapViaAccumulate(_f: (x: number) => number, _sequence: List<number>): List<number> {
  throw new PendingSolution();
}

/** The book's append, rebuilt as an accumulation over `seq1`. */
export function appendViaAccumulate(_seq1: List<number>, _seq2: List<number>): List<number> {
  throw new PendingSolution();
}

/** The book's length, rebuilt as an accumulation. */
export function lengthViaAccumulate(_sequence: List<number>): number {
  throw new PendingSolution();
}
