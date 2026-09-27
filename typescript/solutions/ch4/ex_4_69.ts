// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { makeQueryEngine } from "../../packages/ch4/src/04-logic.js";
import { genealogy } from "./ex_4_63.js";

/**
 * The greats rules over the 4.63 database: descend one son step from ?x,
 * then re-ask the rest of the relationship through the variable-led
 * call. A list-headed call unifies against no ordinary conclusion, so
 * the grandson anchor rule catches ((grandson) ?x ?y) and grounds the
 * recursion.
 */
export const greatRules = [
  "(rule ((grandson) ?x ?y) (grandson ?x ?y))",
  "(rule ((great . ?relation) ?x ?y) (and (son ?x ?child) (?relation ?child ?y)))",
];

/** Answers one greats query over the 4.63 database plus the greats rules. */
export const familyAnswers = (query: string, limit = 20): ReadonlyArray<string> => {
  const engine = makeQueryEngine();
  engine.load([genealogy, ...greatRules]);
  return engine.answers(query, limit);
};

export function ex_4_69(): string {
  const [great] = familyAnswers("((great grandson) Adam ?who)");
  const [fifth] = familyAnswers("((great great great great great grandson) Adam ?who)");
  return `Adam's great-grandson is ${great}; at five greats the line reaches ${fifth}.`;
}
