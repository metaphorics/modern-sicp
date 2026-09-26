// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";

export const nextToRules = `
(rule (?x next-to ?y in (?x ?y . ?u)))
(rule (?x next-to ?y in (?v . ?z))
  (?x next-to ?y in ?z))
`;

export const nextToAnswers = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const engine = microshaft();
  engine.load(nextToRules);
  return [
    engine.answers("(?x next-to ?y in (1 (2 3) 4))"),
    engine.answers("(?x next-to 1 in (2 1 3 1))"),
  ];
};

export function ex_4_61(): string {
  const [first, second] = nextToAnswers();
  return `The first query yields ${first.join("; ")}; the second yields ${second.join("; ")}.`;
}
