// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.13: add an explicit `unbind` extension to the guest language.
 * Specify which frame loses the binding and what happens when no frame
 * has it. Implement the evaluator-side `removeBinding` operation and
 * show the shadowing behavior: after unbinding an inner
 * binding the outer one is visible again, and unbinding a name that is
 * bound nowhere is an error.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.13 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The exercise's typed remove-binding operation. */
export type UnbindRequest = { readonly name: string };
export const unbindRequest: UnbindRequest = { name: "x" };

/** Typed environment steps. Removing the inner binding makes the outer
 * binding visible again; removing a missing name is an error. */
export type BindingFixture = { readonly name: string; readonly value: number };
export type ShadowStep =
  | { readonly kind: "enter-frame"; readonly frame: "outer" | "inner" }
  | { readonly kind: "bind"; readonly frame: "outer" | "inner"; readonly binding: BindingFixture }
  | { readonly kind: "remove"; readonly frame: "outer" | "inner"; readonly request: UnbindRequest }
  | { readonly kind: "read"; readonly frame: "outer" | "inner"; readonly name: string };

export const shadowSteps: readonly ShadowStep[] = [
  { kind: "enter-frame", frame: "outer" },
  { kind: "bind", frame: "outer", binding: { name: "a", value: 1 } },
  { kind: "enter-frame", frame: "inner" },
  { kind: "bind", frame: "inner", binding: { name: "a", value: 2 } },
  { kind: "remove", frame: "inner", request: { name: "a" } },
  { kind: "read", frame: "outer", name: "a" },
  { kind: "remove", frame: "outer", request: { name: "missing" } },
];

export function ex_4_13(): string {
  throw new PendingSolution();
}
