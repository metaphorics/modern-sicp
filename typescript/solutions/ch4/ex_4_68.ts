// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { makeQueryEngine } from "../../packages/ch4/src/04-logic.js";

/** append-to-form and reverse as rules, the book's 4.68 database. */
export const reverseRules = [
  "(rule (append-to-form () ?y ?y))",
  "(rule (append-to-form (?u . ?v) ?y (?u . ?z)) (append-to-form ?v ?y ?z))",
  "(rule (reverse () ()))",
  "(rule (reverse (?u . ?v) ?y) (and (reverse ?v ?z) (append-to-form ?z (?u) ?y)))",
];

/** Answers one reverse query; the backward query takes only its first answer. */
export const reverseAnswers = (query: string, limit = 8): ReadonlyArray<string> => {
  const engine = makeQueryEngine();
  engine.load(reverseRules);
  return engine.answers(query, limit);
};

export function ex_4_68(): string {
  const [forward] = reverseAnswers("(reverse (1 2 3) ?x)");
  return `Forward reversal derives ${forward}; the backward query derives (3 2 1) first and then keeps generating, so it is sampled with a limit of one.`;
}
