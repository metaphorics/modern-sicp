// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.58: the big-shot rule. A person is a big shot in a
 * division when they work in the division but no supervisor of theirs
 * works in it. The division patterns are dotted tails, so multi-word
 * job titles match whatever follows the division word.
 */
import {
  type Query,
  qpair,
  qtext,
  queryAtom,
  qvar,
  type Rule,
  rule,
} from "../../packages/ch4/src/04-logic.js";
import { answerLines, microshaftDatabase } from "./ex_4_55.js";

/** A big shot works in the division with no supervisor in it. */
export const bigShotRule: Rule = rule(queryAtom("big-shot", qvar("person"), qvar("division")), {
  tag: "and",
  clauses: [
    queryAtom("job", qvar("person"), qpair(qvar("division"), qvar("job-rest"))),
    {
      tag: "not",
      clause: {
        tag: "and",
        clauses: [
          queryAtom("supervisor", qvar("person"), qvar("boss")),
          queryAtom("job", qvar("boss"), qpair(qvar("division"), qvar("boss-rest"))),
        ],
      },
    },
  ],
});

/** Every big shot with their division. */
export const bigShots: Query = queryAtom("big-shot", qvar("person"), qvar("division"));

/** The answer lines for the big-shot query, in evaluator order. */
export const bigShotAnswers = (): ReadonlyArray<string> => {
  const db = microshaftDatabase();
  db.addRule(bigShotRule);
  return answerLines(db, bigShots);
};

export function ex_4_58(): string {
  const answers = bigShotAnswers();
  return `${answers.length} big shots run their divisions.`;
}
