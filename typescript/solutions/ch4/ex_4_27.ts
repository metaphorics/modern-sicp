// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.27: lazy identity with set!. The sequence is argued from
 * the delay rules and pinned with the driver transcript. Defining `w`
 * applies the outer identity to the delayed `(id 10)`: evaluating the
 * outer body runs its count increment and its last expression answers
 * the inner thunk, because a delay node does not evaluate. Asking for
 * `w` is a demand site: the thunk forces, the inner increment runs, and
 * 10 fills the memo cell. The re-display forces the same memoized cell
 * and adds nothing, which is the demonstration that the section's
 * evaluator memoizes.
 */
import { runLazySource } from "../../packages/ch4/src/02-lazy.js";

/** The session: the counter observes each identity call exactly once. */
export const lazyIdSource = `
let count = 0;
const id = (x: number): number => {
  count = count + 1;
  return x;
};
const w = id(delay(id(10)));
console.log(count);
console.log(force(w));
console.log(count);
console.log(force(w));
console.log(count);
`;

/** The pinned sequence through the named lazy experiment. */
export const answers = (): RunResult => runLazySource(lazyIdSource, "lazy-memoized-experiment");

export function ex_4_27(): string {
  return (
    "Defining w applies the outer identity to the delayed (id 10): the outer body runs its " +
    "increment (count 1) and answers the thunk unevaluated. Asking for w forces: the inner " +
    "increment runs (count 2) and 10 fills the memo cell. Re-displaying w forces the same " +
    "memoized cell and adds nothing — the demonstration that the section's evaluator " +
    "memoizes. The sequence is 1, 10, 2, 10, 2."
  );
}
