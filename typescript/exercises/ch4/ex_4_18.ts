// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.18: the alternative scan-out strategy. Each declaration's value
 * expression is computed in an inner local block before any assignment runs. The
 * statement asks whether the solve procedure of 3.5.4 works under this
 * scan, and whether it works under the text's scan, and why.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.18 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Typed solve premise: the two bindings read each other. */
export type SolveBinding = {
  readonly name: "y" | "dy";
  readonly dependsOn: ReadonlyArray<"y" | "dy">;
};
export const solveBindings: readonly SolveBinding[] = [
  { name: "y", dependsOn: ["dy"] },
  { name: "dy", dependsOn: ["y"] },
];

/** The alternative's typed order: compute values before writes. */
export type AlternativeStep =
  | { readonly kind: "declare-unassigned"; readonly name: "u" | "v" }
  | { readonly kind: "compute"; readonly name: "a" | "b"; readonly source: "e1" | "e2" }
  | { readonly kind: "write"; readonly name: "u" | "v"; readonly source: "a" | "b" }
  | { readonly kind: "read"; readonly name: "e3" };
export const alternativeSteps: readonly AlternativeStep[] = [
  { kind: "declare-unassigned", name: "u" },
  { kind: "declare-unassigned", name: "v" },
  { kind: "compute", name: "a", source: "e1" },
  { kind: "compute", name: "b", source: "e2" },
  { kind: "write", name: "u", source: "a" },
  { kind: "write", name: "v", source: "b" },
  { kind: "read", name: "e3" },
];

export function ex_4_18(): string {
  throw new PendingSolution();
}
