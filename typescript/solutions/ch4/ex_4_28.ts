// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.28: forcing the operator. `force(delay(add))(2, 3)` is
 * the demonstration: the operator position holds a thunk whose
 * expression is `add`, so the application must force the operator
 * before apply can dispatch, and the forced call answers 5. The
 * negation proves the point: leaving the operator unforced hands the
 * thunk itself to apply, which answers the typed `not-callable` fault
 * — the thunk renders as `#[thunk]` in the engine's neutral display.
 */
import { runLazySource } from "../../packages/ch4/src/02-lazy.js";

/** The forced variant: the operator thunk is forced before application. */
export const forcedOperatorSource = `
const add = (a: number, b: number): number => a + b;
console.log(force(delay(add))(2, 3));
`;

/** The unforced variant: the thunk itself reaches apply. */
export const unforcedOperatorSource = `
const add = (a: number, b: number): number => a + b;
console.log(delay(add)(2, 3));
`;

/** The forced run through the named lazy experiment. */
export const runForcedOperator = (): RunResult =>
  runLazySource(forcedOperatorSource, "lazy-memoized-experiment");

/** The unforced run through the named lazy experiment. */
export const runUnforcedOperator = (): RunResult =>
  runLazySource(unforcedOperatorSource, "lazy-memoized-experiment");

export function ex_4_28(): string {
  return (
    "The operator position holds a thunk, so the application clause must run the actual " +
    "value of the operator before apply can dispatch: `force(delay(add))(2, 3)` answers 5. " +
    "The unforced variant hands the thunk itself to apply and answers the typed " +
    "not-callable fault, with the thunk rendered #[thunk] in the neutral display."
  );
}
