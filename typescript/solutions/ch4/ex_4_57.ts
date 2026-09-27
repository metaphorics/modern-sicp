// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";
import type { Value } from "../../packages/ch4/src/core.js";

export const replacementRules = `
(rule (same ?x ?x))
(rule (can-replace ?person-1 ?person-2)
  (and (job ?person-1 ?job-1)
       (job ?person-2 ?job-2)
       (or (same ?job-1 ?job-2)
           (can-do-job ?job-1 ?job-2))
       (not (same ?person-1 ?person-2))))
`;

const lessThan = (args: ReadonlyArray<Value>): boolean => {
  const [left, right] = args;
  return left?._tag === "Number" && right?._tag === "Number" && left.n < right.n;
};

export const replacementAnswers = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const engine = microshaft();
  engine.load(replacementRules);
  engine.setPredicates({ "<": lessThan });
  return [
    engine.answers("(can-replace ?person (Fect Cy D))"),
    engine.answers(
      "(and (can-replace ?person-1 ?person-2) (salary ?person-1 ?salary-1) (salary ?person-2 ?salary-2) (lisp-value < ?salary-1 ?salary-2))",
    ),
  ];
};

export function ex_4_57(): string {
  const [cyReplacements, lowerPaid] = replacementAnswers();
  return `The rule excludes self-replacement. It finds ${cyReplacements.length} replacements for Cy and ${lowerPaid.length} lower-paid replacement relationships.`;
}
