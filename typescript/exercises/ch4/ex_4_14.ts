// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.14: Louis Reasoner installs the host's map as an evaluator
 * primitive. The demand: show by running code which calls work and which
 * fail (a map that cannot call evaluator procedures dies on compound
 * procedures), then build Eva Lu Ator's version, whose map applies
 * evaluator procedures through the evaluator's own apply, and show every
 * call works there.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.14 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Typed call premises: primitive and guest-defined procedures. */
export type MapOperand =
  | { readonly kind: "primitive"; readonly name: "first" }
  | { readonly kind: "guest"; readonly operation: "identity" | "square" };
export type MapCall = {
  readonly operand: MapOperand;
  readonly input: ReadonlyArray<readonly [number, number]> | ReadonlyArray<number>;
};
export const mapCalls: readonly MapCall[] = [
  {
    operand: { kind: "primitive", name: "first" },
    input: [
      [1, 2],
      [3, 4],
    ],
  },
  { operand: { kind: "guest", operation: "identity" }, input: [[9, 10]] },
  { operand: { kind: "guest", operation: "square" }, input: [1, 2, 3] },
];

export function ex_4_14(): string {
  throw new PendingSolution();
}
