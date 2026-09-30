// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.2: reorder the cases in `evaluate` so the call case precedes
 * the assignment case. (a) Explain the failure: the declaration
 * `const x = 3;` is not a call expression and cannot be dispatched as one.
 * (b) Change the tagged syntax so application nodes carry an explicit
 * `call` tag. Typed premise data represents both `factorial(3)` and
 * `square(x) + 1` as call nodes plus arithmetic, without introducing a
 * second source grammar.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.2 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Typed syntax premise for the explicit call-tag examples. */
export type DispatchNode =
  | { readonly kind: "call"; readonly callee: string; readonly arguments: readonly number[] }
  | { readonly kind: "add"; readonly left: DispatchNode; readonly right: number };
export const factorialCall: DispatchNode = { kind: "call", callee: "factorial", arguments: [3] };
export const squareCallPlusOne: DispatchNode = {
  kind: "add",
  left: { kind: "call", callee: "square", arguments: [1] },
  right: 1,
};

export function ex_4_02(): string {
  throw new PendingSolution();
}
