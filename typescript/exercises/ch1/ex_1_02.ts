// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.2: write the book's fraction as one nested application of
 * the four primitives below, with every operator written as a call, and
 * give its value.
 *
 * The primitives are given by the statement; the pending part is the
 * nested application itself.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.2 is not solved yet");
    this.name = "PendingSolution";
  }
}

export const add = (x: number, y: number): number => x + y;

export const sub = (x: number, y: number): number => x - y;

export const mul = (x: number, y: number): number => x * y;

export const div = (x: number, y: number): number => x / y;

export function ex_1_02(): number {
  throw new PendingSolution();
}
