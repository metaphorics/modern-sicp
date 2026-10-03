// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.20: recursive local bindings as a derived expression. Part (a)
 * transforms a recursive binding block into a local block that pre-binds the
 * names to *unassigned* and then assigns them with TypeScript assignment.
 * Part (b) asks what is loose about Louis's claim that a plain local binding
 * can replace recursive bindings.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.20 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Typed mutually recursive binding premise. */
export type RecursiveBinding = {
  readonly name: "even" | "odd";
  readonly base: boolean;
  readonly calls: "odd" | "even";
};
export const recursiveBindings: readonly RecursiveBinding[] = [
  { name: "even", base: true, calls: "odd" },
  { name: "odd", base: false, calls: "even" },
];

export function ex_4_20(): string {
  throw new PendingSolution();
}
