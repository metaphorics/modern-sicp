// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.19: Ben, Alyssa, and Eva debate the result of the program
 * below. The statement asks which viewpoint (if any) to support and how to
 * implement internal definitions so they behave as Eva prefers.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.19 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Typed definition-order premise the three are arguing about. */
export type DebateBinding = {
  readonly scope: "outer" | "function";
  readonly name: "a" | "b";
  readonly initializer: "1" | "5" | "outerAPlusX";
  readonly readsBeforeDeclaration: ReadonlyArray<"a">;
};
export const debatedBindings: readonly DebateBinding[] = [
  { scope: "outer", name: "a", initializer: "1", readsBeforeDeclaration: [] },
  { scope: "function", name: "b", initializer: "outerAPlusX", readsBeforeDeclaration: ["a"] },
  { scope: "function", name: "a", initializer: "5", readsBeforeDeclaration: [] },
];

export function ex_4_19(): string {
  throw new PendingSolution();
}
