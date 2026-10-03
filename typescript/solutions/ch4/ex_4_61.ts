// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Database } from "../../packages/ch4/src/04-logic.js";
/**
 * Exercise 4.61: next-to as atoms. The book's list-pattern rules
 * become explicit three-place atoms over list terms: the base rule
 * names the head pair, the step rule strips one element and recurses.
 * Both book queries run to completion over finite lists.
 */
import {
  makeDatabase,
  type Query,
  qlist,
  qpair,
  qtext,
  queryAtom,
  qvar,
  type Rule,
  rule,
} from "../../packages/ch4/src/04-logic.js";
import { answerLines } from "./ex_4_55.js";

/** Adjacent elements: head pair, or adjacency in the tail. */
export const nextToRules: ReadonlyArray<Rule> = [
  rule(queryAtom("next-to", qvar("x"), qvar("y"), qpair(qvar("x"), qpair(qvar("y"), qvar("u"))))),
  rule(
    queryAtom("next-to", qvar("x"), qvar("y"), qpair(qvar("v"), qvar("z"))),
    queryAtom("next-to", qvar("x"), qvar("y"), qvar("z")),
  ),
];

/** Adjacent pairs of (1 (2 3) 4). */
export const adjacentInMixed: Query = queryAtom(
  "next-to",
  qvar("x"),
  qvar("y"),
  qlist(qtext("1"), qlist(qtext("2"), qtext("3")), qtext("4")),
);

/** Elements immediately before a 1 in (2 1 3 1). */
export const beforeOne: Query = queryAtom(
  "next-to",
  qvar("x"),
  qtext("1"),
  qlist(qtext("2"), qtext("1"), qtext("3"), qtext("1")),
);

/** The answer lines for both adjacency queries, in evaluator order. */
export const nextToAnswers = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const db: Database = makeDatabase();
  for (const candidate of nextToRules) {
    db.addRule(candidate);
  }
  return [answerLines(db, adjacentInMixed), answerLines(db, beforeOne)];
};

export function ex_4_61(): string {
  const [mixed, before] = nextToAnswers();
  return (
    `Adjacency finds ${mixed.length} pairs in the mixed list and ` +
    `${before.length} predecessors of 1.`
  );
}
