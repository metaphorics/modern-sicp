// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";

const queries = [
  "(supervisor ?person (Bitdiddle Ben))",
  "(job ?person (accounting . ?job))",
  "(address ?person (Slumerville . ?address))",
] as const;

/** Answers to the three database lookups, retaining evaluator order. */
export const simpleQueryAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const engine = microshaft();
  return [engine.answers(queries[0]), engine.answers(queries[1]), engine.answers(queries[2])];
};

export function ex_4_55(): string {
  const [supervisees, accountants, residents] = simpleQueryAnswers();
  return `Simple queries return ${supervisees.length} people supervised by Ben, ${accountants.length} accounting employees, and ${residents.length} Slumerville addresses.`;
}
