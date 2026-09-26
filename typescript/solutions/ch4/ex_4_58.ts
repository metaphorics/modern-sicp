// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";

export const bigShotRule = `
(rule (big-shot ?person ?division)
  (and (job ?person (?division . ?title))
       (not (and (supervisor ?person ?boss)
                 (job ?boss (?division . ?boss-title)))))
)
`;

export const bigShots = (): ReadonlyArray<string> => {
  const engine = microshaft();
  engine.load(bigShotRule);
  return engine.answers("(big-shot ?person ?division)");
};

export function ex_4_58(): string {
  return `A person is a big shot when their supervisor is absent from their own division: ${bigShots().join("; ")}.`;
}
