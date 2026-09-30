// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.9: many languages offer iteration constructs such as do, for,
 * while, and until, while the evaluated language expresses iteration with ordinary
 * procedure calls. The demand: design and implement iteration constructs
 * for the evaluator as derived expressions (or otherwise), without
 * cheating through the host's apply boundary. The pending part is
 * the constructs themselves plus evidence that they run iterative
 * processes, here summing loops checked against manual recursion.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.9 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Checked TypeScript-subset iteration fixtures. */
export const summingWhileProgram = `let total = 0;
let i = 1;
while (i < 5) {
  total = total + i;
  i = i + 1;
}
total;`;

/** A for-of loop that accumulates 1 through 5 into total. */
export const summingForProgram = `let total = 0;
for (const n of [1, 2, 3, 4, 5]) {
  total = total + n;
}
total;`;

/** The same sum written as manual recursion, the standard it must match. */
export const manualSumProgram = `function sumTo(n: number): number {
  if (n === 0) {
    return 0;
  }
  return n + sumTo(n - 1);
}
sumTo(5);`;

export function ex_4_09(): string {
  throw new PendingSolution();
}
