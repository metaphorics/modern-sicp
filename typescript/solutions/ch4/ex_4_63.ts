// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.63: family rules. The Genesis database traces Adam's
 * line; the grandson rule chains two son links, and the wife's-son
 * rule makes a wife's son her husband's son. Queries: Cain's
 * grandson, Lamech's sons, Methushael's grandsons.
 */
import {
  type Database,
  makeDatabase,
  type Query,
  qtext,
  queryAtom,
  qvar,
  type Rule,
  rule,
} from "../../packages/ch4/src/04-logic.js";
import { answerLines } from "./ex_4_55.js";

/** The Genesis database, in the book's assertion order. */
export const genealogyDatabase = (): Database => {
  const db: Database = makeDatabase();
  const facts: ReadonlyArray<Query> = [
    queryAtom("son", qtext("Adam"), qtext("Cain")),
    queryAtom("son", qtext("Cain"), qtext("Enoch")),
    queryAtom("son", qtext("Enoch"), qtext("Irad")),
    queryAtom("son", qtext("Irad"), qtext("Mehujael")),
    queryAtom("son", qtext("Mehujael"), qtext("Methushael")),
    queryAtom("son", qtext("Methushael"), qtext("Lamech")),
    queryAtom("wife", qtext("Lamech"), qtext("Ada")),
    queryAtom("son", qtext("Ada"), qtext("Jabal")),
    queryAtom("son", qtext("Ada"), qtext("Jubal")),
  ];
  for (const fact of facts) {
    db.addAssertion(fact);
  }
  return db;
};

/** A grandson links two son links; a wife's son is her husband's. */
export const familyRules: ReadonlyArray<Rule> = [
  rule(queryAtom("grandson", qvar("grandson"), qvar("grandfather")), {
    tag: "and",
    clauses: [
      queryAtom("son", qvar("father"), qvar("grandson")),
      queryAtom("son", qvar("grandfather"), qvar("father")),
    ],
  }),
  rule(queryAtom("son", qvar("husband"), qvar("son")), {
    tag: "and",
    clauses: [
      queryAtom("wife", qvar("husband"), qvar("wife")),
      queryAtom("son", qvar("wife"), qvar("son")),
    ],
  }),
];

/** Cain's grandson, Lamech's sons, Methushael's grandsons. */
export const cainsGrandson: Query = queryAtom("grandson", qvar("grandson"), qtext("Cain"));
export const lamechsSons: Query = queryAtom("son", qtext("Lamech"), qvar("son"));
export const methusshaelsGrandsons: Query = queryAtom(
  "grandson",
  qvar("grandson"),
  qtext("Methushael"),
);
/** The answer lines for all three family queries, in evaluator order. */
export const familyAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const db = genealogyDatabase();
  for (const candidate of familyRules) {
    db.addRule(candidate);
  }
  return [
    answerLines(db, cainsGrandson),
    answerLines(db, lamechsSons),
    answerLines(db, methusshaelsGrandsons),
  ];
};

export function ex_4_63(): string {
  const [cain, lamech, methushael] = familyAnswers();
  return (
    `Cain has ${cain.length} grandson; Lamech has ${lamech.length} sons; ` +
    `Methushael has ${methushael.length} grandsons.`
  );
}
