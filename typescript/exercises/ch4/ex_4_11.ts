// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.11: represent a frame as an array of name-cell bindings
 * searched in order instead of a map; each binding is a name-value pair.
 * Rewrite the environment operations. The pending part is the frame
 * representation and the new lookupVariableValue, setVariableValue, and
 * defineVariableValue procedures that walk it.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.11 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A frame entry points at the shared cell every alias observes. */
export type CellFixture = { value: number | undefined };
export type BindingFixture = { readonly name: string; readonly cell: CellFixture };
export const sampleFrame: readonly BindingFixture[] = [
  { name: "a", cell: { value: 1 } },
  { name: "b", cell: { value: 2 } },
];

/** Checked source programs exercising declaration, lookup, assignment, and
 * shared-frame visibility through a nested frame. */
export const assocPrograms: readonly string[] = [
  `const a = 1;
a;`,
  `const a = 1;
const b = 2;
b;`,
  `let a = 1;
const result = (() => {
  a = 10;
  return a;
})();
result;`,
];

export function ex_4_11(): string {
  throw new PendingSolution();
}
