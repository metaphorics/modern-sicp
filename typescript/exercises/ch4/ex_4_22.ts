// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.22: extend the analyzed evaluator of 4.1.7 with the special
 * form let (exercise 4.6), so a local binding is analyzed into the
 * execution procedure of its function call.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.22 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Typed call-based lowering the analyzed evaluator should accept. */
export type LetLowering =
  | { readonly kind: "bind"; readonly parameter: string; readonly initializer: number }
  | { readonly kind: "call"; readonly callee: string; readonly argument: number }
  | { readonly kind: "result"; readonly value: number };
export const letLowering: readonly LetLowering[] = [
  { kind: "bind", parameter: "x", initializer: 3 },
  { kind: "call", callee: "body", argument: 3 },
  { kind: "result", value: 7 },
];

export function ex_4_22(): string {
  throw new PendingSolution();
}
