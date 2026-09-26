// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";

export const lastPairRules = `
(rule (last-pair (?x) (?x)))
(rule (last-pair (?x . ?tail) ?result)
  (last-pair ?tail ?result))
`;
export const lastPairAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const engine = microshaft();
  engine.load(lastPairRules);
  return [
    engine.answers("(last-pair (3) ?x)"),
    engine.answers("(last-pair (1 2 3) ?x)"),
    engine.answers("(last-pair (2 ?x) (3))"),
  ];
};

export function ex_4_62(): string {
  const [single, three, partial] = lastPairAnswers();
  return `Forward and partially specified queries return ${single.length}, ${three.length}, and ${partial.length} answers. Asking for an unconstrained input ending in (3) can recurse through ever longer candidate lists, so this left-recursive rule is not a terminating reverse search.`;
}
