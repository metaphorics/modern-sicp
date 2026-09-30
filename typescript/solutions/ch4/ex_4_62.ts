// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.62: last-pair rules. A singleton's last pair is itself;
 * a longer list's last pair is its tail's last pair. The first three
 * book queries terminate; the open-ended fourth enumerates ever
 * longer pre-images without end, so it is exhibited but never run.
 * Addition 4.62a implements the inverter gate as two ground rules and
 * checks both signal directions.
 */
import {
  type Database,
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

/** The last pair of a list, by singleton base and tail step. */
export const lastPairRules: ReadonlyArray<Rule> = [
  rule(queryAtom("last-pair", qpair(qvar("x"), { tag: "nil" }), qpair(qvar("x"), { tag: "nil" }))),
  rule(
    queryAtom("last-pair", qpair(qvar("u"), qvar("v")), qvar("x")),
    queryAtom("last-pair", qvar("v"), qvar("x")),
  ),
];

/** The last pair of the singleton (3). */
export const lastOfSingleton: Query = queryAtom("last-pair", qlist(qtext("3")), qvar("x"));

/** The last pair of (1 2 3). */
export const lastOfThree: Query = queryAtom(
  "last-pair",
  qlist(qtext("1"), qtext("2"), qtext("3")),
  qvar("x"),
);

/** Which middle makes (2 ?) end in (3). */
export const lastConstrained: Query = queryAtom(
  "last-pair",
  qlist(qtext("2"), qvar("x")),
  qlist(qtext("3")),
);

/**
 * The open-ended query (last-pair ?x (3)): rule heads keep generating
 * longer pre-images, so the answer set is infinite and the driver
 * would never return. Exhibited, never executed (UNRUN).
 */
export const lastOpenEnded: Query = queryAtom("last-pair", qvar("x"), qlist(qtext("3")));

/** The answer lines for the three terminating queries, in order. */
export const lastPairAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const db: Database = makeDatabase();
  for (const candidate of lastPairRules) {
    db.addRule(candidate);
  }
  return [
    answerLines(db, lastOfSingleton),
    answerLines(db, lastOfThree),
    answerLines(db, lastConstrained),
  ];
};

/** Addition 4.62a: the inverter gate as two ground rules. */
export const inverterRules: ReadonlyArray<Rule> = [
  rule(queryAtom("logic-not", qtext("0"), qtext("1"))),
  rule(queryAtom("logic-not", qtext("1"), qtext("0"))),
];

/** Both inverter directions. */
export const inverterAnswers = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const db: Database = makeDatabase();
  for (const candidate of inverterRules) {
    db.addRule(candidate);
  }
  return [
    answerLines(db, queryAtom("logic-not", qtext("0"), qvar("out"))),
    answerLines(db, queryAtom("logic-not", qvar("input"), qtext("0"))),
  ];
};

export function ex_4_62(): string {
  const [singleton, three, constrained] = lastPairAnswers();
  const [low, high] = inverterAnswers();
  return (
    `Last pairs answer ${singleton.length + three.length + constrained.length} ` +
    `terminating queries; the inverter answers ${low.length + high.length} directions.`
  );
}
