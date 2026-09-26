// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  type Frame,
  instantiate,
  isVar,
  type makeQueryEngine,
} from "../../packages/ch4/src/04-logic.js";
import type { Value } from "../../packages/ch4/src/core.js";
export type DelayedNot = { predicate: Value; variables: ReadonlyArray<Value> };
export const delayNot = (predicate: Value, variables: ReadonlyArray<Value>): DelayedNot => ({
  predicate,
  variables,
});
export const ready = (frame: Frame, filter: DelayedNot): boolean =>
  filter.variables.every((variable) => !isVar(instantiate(variable, frame, (unbound) => unbound)));
export function runDelayedNot(
  engine: ReturnType<typeof makeQueryEngine>,
  frame: Frame,
  filter: DelayedNot,
): boolean {
  if (!ready(frame, filter)) throw new Error("not waits for variable bindings");
  return engine.query(filter.predicate, frame).take(1).length === 0;
}
export function ex_4_77(): string {
  return "Defer negation until all variables in its predicate are bound, then filter by query failure.";
}
