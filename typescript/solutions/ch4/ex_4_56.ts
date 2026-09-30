// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.56: compound queries. Three combinations over the personnel
 * database: supervisees with addresses (conjunction), lower salaries
 * (conjunction with a host predicate over the instantiated salary
 * texts), and supervisees of non-computer supervisors (conjunction with
 * negation as failure). Addition 4.56a adds a second negated compound:
 * Ben's supervisees who are not computer programmers.
 */
import {
  type Query,
  qlist,
  qpair,
  qtext,
  queryAtom,
  qvar,
} from "../../packages/ch4/src/04-logic.js";
import type { Value } from "../../packages/ch4/src/runtime/value.js";
import { answerLines, microshaftDatabase } from "./ex_4_55.js";

/** Ben's supervisees together with their addresses. */
export const superviseesWithAddresses: Query = {
  tag: "and",
  clauses: [
    queryAtom("supervisor", qvar("person"), qlist(qtext("Bitdiddle"), qtext("Ben"))),
    queryAtom("address", qvar("person"), qvar("where")),
  ],
};

/** True when the first salary text is numerically below the second. */
export const salaryLess = (args: ReadonlyArray<Value>): boolean => {
  if (args.length !== 2) {
    return false;
  }
  const [low, high] = [args[0], args[1]];
  return typeof low === "string" && typeof high === "string" ? Number(low) < Number(high) : false;
};

/** Everyone earning less than Ben, with both salaries. */
export const lowerSalaries: Query = {
  tag: "and",
  clauses: [
    queryAtom("salary", qvar("person"), qvar("amount")),
    queryAtom("salary", qlist(qtext("Bitdiddle"), qtext("Ben")), qvar("ben-amount")),
    {
      tag: "lisp-value",
      predicate: salaryLess,
      args: [qvar("amount"), qvar("ben-amount")],
    },
  ],
};

/** Supervisees of non-computer supervisors, with supervisor name and job. */
export const supervisedOutsideComputer: Query = {
  tag: "and",
  clauses: [
    queryAtom("supervisor", qvar("person"), qvar("supervisor")),
    {
      tag: "not",
      clause: queryAtom("job", qvar("supervisor"), qpair(qtext("computer"), qvar("division-rest"))),
    },
    queryAtom("job", qvar("supervisor"), qvar("supervisor-job")),
  ],
};

/** Addition 4.56a: Ben's supervisees who are not computer programmers. */
export const nonProgrammersSupervisedByBen: Query = {
  tag: "and",
  clauses: [
    queryAtom("supervisor", qvar("person"), qlist(qtext("Bitdiddle"), qtext("Ben"))),
    {
      tag: "not",
      clause: queryAtom("job", qvar("person"), qlist(qtext("computer"), qtext("programmer"))),
    },
  ],
};

/** The answer lines for each compound query, in evaluator order. */
export const compoundAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const db = microshaftDatabase();
  return [
    answerLines(db, superviseesWithAddresses),
    answerLines(db, lowerSalaries),
    answerLines(db, supervisedOutsideComputer),
    answerLines(db, nonProgrammersSupervisedByBen),
  ];
};

export function ex_4_56(): string {
  const [addresses, salaries, outside, nonProgrammers] = compoundAnswers();
  return (
    `Compound queries return ${addresses.length} supervisee addresses, ` +
    `${salaries.length} lower salaries, ${outside.length} people supervised outside ` +
    `the computer division, and ${nonProgrammers.length} non-programmer supervisee of Ben.`
  );
}
