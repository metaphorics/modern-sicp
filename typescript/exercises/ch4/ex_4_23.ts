// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.23: compare the text's analyze-sequence, which folds the
 * execution procedures into one at analysis time, with Alyssa's version,
 * which keeps a list and loops through it at run time. The statement asks
 * what work each does for a one-statement body and for a two-statement
 * body.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.23 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Alyssa's expected shape as typed evaluator steps: analyze each
 * expression, then loop through the procedures at run time. */
export type AnalysisStep =
  | { readonly kind: "analyze-each"; readonly count: number }
  | { readonly kind: "execute-sequence"; readonly count: number };
export const alyssaShapeSteps: readonly AnalysisStep[] = [
  { kind: "analyze-each", count: 2 },
  { kind: "execute-sequence", count: 2 },
];

export function ex_4_23(): string {
  throw new PendingSolution();
}
