// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.71: why simple-query and disjoin delay. The delayed
 * versions interleave assertion hits with rule hits; the no-delay
 * variants append, so every assertion answer precedes every rule
 * answer — and behind an unbounded assertion stream the rules would
 * starve. Both orders are pinned for real over the Genesis sons:
 * delayed interleaves the wife-rule's Lamech sons among the
 * asserted sons, while no-delay lists all eight assertions first.
 * The disjoin pair shows the same split across two supervisor
 * branches. Seed streams are built as stream literals, since the
 * driver owns the only other seed.
 */
import {
  disjoin,
  disjoinNoDelay,
  type Frame,
  formatQuery,
  instantiate,
  mapOverSymbols,
  type Query,
  qlist,
  qtext,
  queryAtom,
  qvar,
  type Stream,
  simpleQuery,
  simpleQueryNoDelay,
  streamToList,
} from "../../packages/ch4/src/04-logic.js";
import { microshaftDatabase } from "./ex_4_55.js";
import { familyRules, genealogyDatabase } from "./ex_4_63.js";

/** The empty-frame seed, built as a stream literal. */
export const seedStream = (): Stream<Frame> => ({ kind: "singleton", value: [] });

/** Instantiates a pattern over realized frames, rendered per line. */
export const linesOf = (pattern: Query, frames: ReadonlyArray<Frame>): string[] =>
  frames.map((frame) =>
    formatQuery(mapOverSymbols(pattern, (term) => instantiate(term, frame, qvar))),
  );

/** Delayed vs appended assertion/rule order over the Genesis sons. */
export const sonOrders = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const db = genealogyDatabase();
  for (const candidate of familyRules) {
    db.addRule(candidate);
  }
  const pattern: Query = queryAtom("son", qvar("s"), qvar("p"));
  const delayed = streamToList(simpleQuery(pattern, seedStream(), db));
  const appended = streamToList(simpleQueryNoDelay(pattern, seedStream(), db));
  return [linesOf(pattern, delayed), linesOf(pattern, appended)];
};

/** Delayed vs appended branch order over two supervisor patterns. */
export const supervisorBranchOrders = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const db = microshaftDatabase();
  const ben = qlist(qtext("Bitdiddle"), qtext("Ben"));
  const hacker = qlist(qtext("Hacker"), qtext("Alyssa"), qtext("P"));
  const clauses: ReadonlyArray<Query> = [
    queryAtom("supervisor", qvar("x"), ben),
    queryAtom("supervisor", qvar("x"), hacker),
  ];
  const pattern: Query = { tag: "or", clauses };
  const delayed = streamToList(disjoin(clauses, seedStream(), db));
  const appended = streamToList(disjoinNoDelay(clauses, seedStream(), db));
  return [linesOf(pattern, delayed), linesOf(pattern, appended)];
};

export function ex_4_71(): string {
  const [delayed, appended] = sonOrders();
  return (
    `Delayed search interleaves rule hits among ${delayed.length} answers; ` +
    `the no-delay variants append all assertions first (${appended.length} ` +
    `answers, rules last). Behind an unbounded assertion stream the ` +
    `appended rules would starve — that is what the delay buys.`
  );
}
