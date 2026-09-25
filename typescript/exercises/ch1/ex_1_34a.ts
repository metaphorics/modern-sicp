// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.34a: the same self-application with the types erased. With
 * `any` the checker stands down and the host fails at runtime when the
 * number 2 is applied as a function.
 */
// biome-ignore lint/suspicious/noExplicitAny: the exercise erases the types on purpose, the host's escape hatch
export const fAny = (g: any): any => g(2);

export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.34a is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The kind and message of the error the host raises for fAny(fAny). */
export function runtimeSelfApplicationError(): { kind: string; message: string } {
  throw new PendingSolution();
}
