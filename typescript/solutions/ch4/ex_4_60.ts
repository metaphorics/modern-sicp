// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";
import type { Value } from "../../packages/ch4/src/core.js";
import { format } from "../../packages/ch4/src/read.js";

export const livesNearRule = `
(rule (same ?x ?x))
(rule (lives-near ?person-1 ?person-2)
  (and (address ?person-1 (?town . ?rest-1))
       (address ?person-2 (?town . ?rest-2))
       (not (same ?person-1 ?person-2))))
`;

export const neighborAnswers = (oneDirection = false): ReadonlyArray<string> => {
  const engine = microshaft();
  engine.load(livesNearRule);
  if (oneDirection) {
    engine.setPredicates({
      "<": (args: ReadonlyArray<Value>) => {
        const [left, right] = args;
        return left !== undefined && right !== undefined && format(left) < format(right);
      },
    });
  }
  const query = oneDirection
    ? "(and (lives-near ?person-1 ?person-2) (lisp-value < ?person-1 ?person-2))"
    : "(lives-near ?person-1 ?person-2)";
  return engine.answers(query);
};

export function ex_4_60(): string {
  return "The rule is symmetric, so either ordering is derived. Add a canonical ordering constraint (for example, compare a stable person key) to retain exactly one orientation of each distinct pair.";
}
