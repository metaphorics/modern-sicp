// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.78: the query language as a nondeterministic program.
 * Each query becomes a search program over the amb evaluator: one
 * answer per success, more on resumption, chosen among the driver's
 * real answer lines embedded as guest data. The bridge runs both
 * directions — a populated query and an empty one — so the
 * behavioral differences show as data: backtracking re-executes
 * where streams memoize, and failure is silent resumption where
 * the driver prints its empty-answer line.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { type Query, qlist, qtext, queryAtom } from "../../packages/ch4/src/04-logic.js";
import { format } from "../../packages/ch4/src/read.js";
import { answerLines, microshaftDatabase, supervisedByBen } from "./ex_4_55.js";

/** The driver's supervisee lines, the bridge's guest data. */
export const superviseeLines = (): ReadonlyArray<string> => {
  const db = microshaftDatabase();
  return [...answerLines(db, supervisedByBen)];
};

/**
 * A search program choosing among embedded lines with a required
 * substring. The alternatives come from real driver output, joined
 * as a guest choice.
 */
export const choiceSource = (lines: ReadonlyArray<string>, required: string): string => {
  const alternatives = lines.map((line) => JSON.stringify(line)).join(", ");
  return [
    `const pick = choose(${alternatives});`,
    `require(pick.includes(${JSON.stringify(required)}));`,
    "pick;",
  ].join("\n");
};

/** The amb answers over the bridge program, rendered. */
export const bridgeAnswers = (required: string): ReadonlyArray<string> => {
  const run = runAmbAnswers(
    choiceSource(superviseeLines(), required),
    "amb-depth-first-experiment",
    1,
  );
  return run.answers.map((value) => format(value));
};

/** Ben supervises nobody so named: a genuinely empty driver query. */
export const selfSupervised: Query = queryAtom(
  "supervisor",
  qlist(qtext("Bitdiddle"), qtext("Ben")),
  qlist(qtext("Bitdiddle"), qtext("Ben")),
);

/** The driver's empty answer against the search's silent one. */
export const emptyComparison = (): readonly [ReadonlyArray<string>, number] => {
  const db = microshaftDatabase();
  const driver = answerLines(db, selfSupervised);
  const run = runAmbAnswers(
    choiceSource(superviseeLines(), "Nobody"),
    "amb-depth-first-experiment",
    1,
  );
  return [driver, run.answers.length];
};

export function ex_4_78(): string {
  const hacker = bridgeAnswers("Hacker");
  const [driverEmpty, searchEmpty] = emptyComparison();
  return (
    `The bridge answers ${hacker.length} Hacker line by search; the empty ` +
    `query prints ${driverEmpty.length} driver line against ${searchEmpty} ` +
    `search answers. Streams memoize derivations, backtracking re-executes ` +
    `them — same answers, different work.`
  );
}
