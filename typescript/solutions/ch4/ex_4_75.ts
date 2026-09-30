// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.75: the unique special form. The engine ships `unique`
 * with `qeval` dispatch: a clause matching exactly one frame keeps
 * that extended frame, anything else empties. Queries: the one
 * computer wizard, the (non-)unique programmers, every singly
 * filled job with its holder, and everyone supervising precisely
 * one person.
 */
import { type Query, qlist, qtext, queryAtom, qvar } from "../../packages/ch4/src/04-logic.js";
import { answerLines, microshaftDatabase } from "./ex_4_55.js";

/** The one computer wizard. */
export const theWizard: Query = {
  tag: "unique",
  clause: queryAtom("job", qvar("x"), qlist(qtext("computer"), qtext("wizard"))),
};

/** The (non-)unique computer programmers. */
export const theProgrammers: Query = {
  tag: "unique",
  clause: queryAtom("job", qvar("x"), qlist(qtext("computer"), qtext("programmer"))),
};

/** Every singly filled job with its holder. */
export const singlyFilledJobs: Query = {
  tag: "and",
  clauses: [
    queryAtom("job", qvar("x"), qvar("j")),
    {
      tag: "unique",
      clause: queryAtom("job", qvar("anyone"), qvar("j")),
    },
  ],
};

/** Everyone supervising precisely one person. */
export const singleSupervisors: Query = {
  tag: "and",
  clauses: [
    queryAtom("supervisor", qvar("y"), qvar("x")),
    {
      tag: "unique",
      clause: queryAtom("supervisor", qvar("anyone"), qvar("x")),
    },
  ],
};

/** The answer lines for all four unique queries, in evaluator order. */
export const uniqueAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const db = microshaftDatabase();
  return [
    answerLines(db, theWizard),
    answerLines(db, theProgrammers),
    answerLines(db, singlyFilledJobs),
    answerLines(db, singleSupervisors),
  ];
};

/** Counts answer lines excluding the driver's empty-answer marker. */
export const groupCount = (lines: ReadonlyArray<string>): number =>
  lines.filter((line) => line !== "No.").length;

export function ex_4_75(): string {
  const [wizard, programmers, singletons, supervisors] = uniqueAnswers();
  return (
    `Unique finds ${groupCount(wizard)} wizard and ${groupCount(programmers)} programmer ` +
    `groups; ${singletons.length} singly filled jobs; ${supervisors.length} ` +
    `single supervisees' bosses.`
  );
}
