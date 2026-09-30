// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.57: the can-replace rule. Person 1 can replace person 2
 * when they share a job, or person 1's job can do person 2's job, and
 * the two are different people. Queries: everyone who can replace Cy
 * D. Fect, and everyone who can replace someone better paid, with both
 * salaries.
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
import { salaryLess } from "./ex_4_56.js";

/** Two people are the same person. */
export const sameRule: Rule = rule(queryAtom("same", qvar("x"), qvar("x")));

/** Replacement through a shared job or a can-do-job step. */
export const canReplaceRule: Rule = rule(
  queryAtom("can-replace", qvar("person-1"), qvar("person-2")),
  {
    tag: "and",
    clauses: [
      {
        tag: "or",
        clauses: [
          {
            tag: "and",
            clauses: [
              queryAtom("job", qvar("person-1"), qvar("job")),
              queryAtom("job", qvar("person-2"), qvar("job")),
            ],
          },
          {
            tag: "and",
            clauses: [
              queryAtom("job", qvar("person-1"), qvar("job-1")),
              queryAtom("job", qvar("person-2"), qvar("job-2")),
              queryAtom("can-do-job", qvar("job-1"), qvar("job-2")),
            ],
          },
        ],
      },
      {
        tag: "not",
        clause: queryAtom("same", qvar("person-1"), qvar("person-2")),
      },
    ],
  },
);

/** Everyone who can replace Cy D. Fect. */
export const fectReplacements: Query = queryAtom(
  "can-replace",
  qvar("person"),
  qlist(qtext("Fect"), qtext("Cy"), qtext("D")),
);

/** Replacements of better-paid people, with both salaries. */
export const replacementsForMore: Query = {
  tag: "and",
  clauses: [
    queryAtom("can-replace", qvar("person-1"), qvar("person-2")),
    queryAtom("salary", qvar("person-1"), qvar("salary-1")),
    queryAtom("salary", qvar("person-2"), qvar("salary-2")),
    {
      tag: "lisp-value",
      predicate: salaryLess,
      args: [qvar("salary-1"), qvar("salary-2")],
    },
  ],
};

/** The answer lines for both replacement queries, in evaluator order. */
export const replacementAnswers = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const db = microshaftDatabase();
  db.addRule(sameRule);
  db.addRule(canReplaceRule);
  return [answerLines(db, fectReplacements), answerLines(db, replacementsForMore)];
};

export function ex_4_57(): string {
  const [fect, more] = replacementAnswers();
  return (
    `${fect.length} people can replace Fect, and ${more.length} replacements ` +
    `favor the better-paid side.`
  );
}
