// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.16: internal declarations bind names that a body's expressions can
 * read before they are assigned. The demand, in three parts: (a) a lookup
 * that fails when it finds the *unassigned* marker; (b) scan internal
 * declarations into one variable-declaration block initialized to
 * `*unassigned*` plus assignments; (c) choose the procedure-construction or
 * closure-body installation point, tested on mutual even/odd functions
 * whose calls appear before their declarations. Native function declarations
 * are hoisted; the evaluator must model that initialization explicitly.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.16 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Typed scan-out witness: closure-valued bindings must create
 * uninitialized cells before their initializers run. */
export type ScanBinding = {
  readonly name: "even" | "odd";
  readonly cellState: "unassigned" | "initialized";
  readonly calls: ReadonlyArray<"even" | "odd">;
};
export const scanBindings: readonly ScanBinding[] = [
  { name: "even", cellState: "unassigned", calls: ["odd"] },
  { name: "odd", cellState: "unassigned", calls: ["even"] },
  { name: "even", cellState: "initialized", calls: ["odd"] },
  { name: "odd", cellState: "initialized", calls: ["even"] },
];

/** Installation points for the scan. Function declarations are hoisted
 * natively; this witness uses closure-valued bindings so initialization
 * order is observable. */
export type InstallPoint = "makeProcedure" | "closureBody";
export const installPoints: readonly InstallPoint[] = ["makeProcedure", "closureBody"];

export function ex_4_16(): string {
  throw new PendingSolution();
}
