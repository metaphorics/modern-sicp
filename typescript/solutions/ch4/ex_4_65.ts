// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";

export const wheelRule = `
(rule (wheel ?person)
  (and (supervisor ?middle-manager ?person)
       (supervisor ?x ?middle-manager)))
`;

export const wheelAnswers = (): ReadonlyArray<string> => {
  const engine = microshaft();
  engine.load(wheelRule);
  return engine.answers("(wheel ?who)");
};

export function ex_4_65(): string {
  const answers = wheelAnswers();
  const oliver = answers.filter((answer) => answer.includes("(Warbucks Oliver)")).length;
  return `Oliver Warbucks is produced ${oliver} times because the wheel rule has four distinct middle-manager/wheel-supervisee proofs for him; these are proof paths, not four distinct people.`;
}
