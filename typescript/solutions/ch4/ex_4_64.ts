// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.64: the outranked-by loop. Louis's recursive-first
 * ordering asks for an outranking through an unconstrained
 * middle-manager before any supervisor pattern binds it, so each
 * answer re-opens the same recursive question forever. The base-first
 * ordering binds a direct supervisor before recursing upward and
 * terminates. The recursive-first rule is exhibited but never run
 * (UNRUN): executing it would hang the driver.
 */
import {
  type Query,
  qlist,
  qtext,
  queryAtom,
  qvar,
  type Rule,
  rule,
} from "../../packages/ch4/src/04-logic.js";
import { answerLines, microshaftDatabase } from "./ex_4_55.js";

/**
 * Louis's recursive-first rule: the recursion precedes the
 * supervisor constraint. Exhibited, never executed (UNRUN).
 */
export const recursiveFirstOutrankedBy: Rule = rule(
  queryAtom("outranked-by", qvar("staff-person"), qvar("boss")),
  {
    tag: "or",
    clauses: [
      queryAtom("supervisor", qvar("staff-person"), qvar("boss")),
      {
        tag: "and",
        clauses: [
          queryAtom("outranked-by", qvar("middle-manager"), qvar("boss")),
          queryAtom("supervisor", qvar("staff-person"), qvar("middle-manager")),
        ],
      },
    ],
  },
);

/** The base-first rule: a direct supervisor binds before the ascent. */
export const baseFirstOutrankedBy: Rule = rule(
  queryAtom("outranked-by", qvar("staff-person"), qvar("boss")),
  {
    tag: "or",
    clauses: [
      queryAtom("supervisor", qvar("staff-person"), qvar("boss")),
      {
        tag: "and",
        clauses: [
          queryAtom("supervisor", qvar("staff-person"), qvar("middle-manager")),
          queryAtom("outranked-by", qvar("middle-manager"), qvar("boss")),
        ],
      },
    ],
  },
);

/** DeWitt's question: who outranks Ben Bitdiddle. */
export const whoOutranksBen: Query = queryAtom(
  "outranked-by",
  qlist(qtext("Bitdiddle"), qtext("Ben")),
  qvar("who"),
);

/** Every outranking pair under the terminating order. */
export const allOutranked: Query = queryAtom("outranked-by", qvar("staff-person"), qvar("boss"));

/**
 * The answer lines under the terminating order, in evaluator order:
 * DeWitt's question first, then the whole relation.
 */
export const outrankedAnswers = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const db = microshaftDatabase();
  db.addRule(baseFirstOutrankedBy);
  return [answerLines(db, whoOutranksBen), answerLines(db, allOutranked)];
};

export function ex_4_64(): string {
  const [ben, all] = outrankedAnswers();
  return (
    `One person outranks Ben; the terminating order lists ${all.length} ` +
    `outrankings. The recursive-first rule loops because the recursion ` +
    `re-opens with an unconstrained middle-manager before any supervisor ` +
    `pattern binds it.`
  );
}
