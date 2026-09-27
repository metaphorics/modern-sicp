// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

export interface DeductionStep {
  readonly pattern: string;
  readonly bindings: Readonly<Record<string, string>>;
}

/** A repeated pattern under the same bindings is the same active subproblem. */
export const repeatsActiveCall = (
  candidate: DeductionStep,
  history: ReadonlyArray<DeductionStep>,
): boolean =>
  history.some(
    (step) =>
      step.pattern === candidate.pattern &&
      Object.keys(step.bindings).length === Object.keys(candidate.bindings).length &&
      Object.entries(step.bindings).every(([name, value]) => candidate.bindings[name] === value),
  );

export function ex_4_67(): string {
  return "Keep a stack of the instantiated query pattern and relevant frame bindings for each active deduction. Before processing a recursive subquery, compare it with active ancestors; if the same pattern has the same bindings, reject that branch. Keep the history branch-local and pop it when a deduction returns, so separate proofs remain available.";
}
