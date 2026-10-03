// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.15: test the diagonal argument for the halting problem.
 * Suppose an oracle decides whether any procedure halts on any input.
 * Apply `try` to itself. If the oracle answers true, `try` runs forever.
 * If the oracle answers false, `try` halts. Build both cases and explain
 * why each observed outcome contradicts the oracle's answer.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.15 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Typed halting premise: both runs apply `try` to itself. */
export type HaltingRun = {
  readonly subject: "try";
  readonly oracleAnswer: boolean;
  readonly observed: "runs-forever" | "halted";
};
export const haltingRuns: readonly HaltingRun[] = [
  { subject: "try", oracleAnswer: true, observed: "runs-forever" },
  { subject: "try", oracleAnswer: false, observed: "halted" },
];

export function ex_4_15(): string {
  throw new PendingSolution();
}
