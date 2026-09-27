// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.34: f applies its argument to 2. The working calls land on 4
 * and 6; the self-application f(f) never runs at all - the type checker
 * rejects it, and its diagnostic is the artifact.
 */
export const f = (g: (n: number) => number): number => g(2);

export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.34 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The two working calls of the statement: [f(square), f((z) => z * (z + 1))]. */
export function workingCalls(): number[] {
  throw new PendingSolution();
}

/** The type checker's diagnostic for the f(f) call, as captured by running tsc. */
export function selfApplicationDiagnostic(): string {
  throw new PendingSolution();
}
