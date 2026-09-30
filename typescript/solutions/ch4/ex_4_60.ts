// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.60: lives-near duplicates. Everyone near Alyssa comes
 * from the shared town variable; the two-variable query lists every
 * directed pair, so each unordered pair appears twice. The
 * name-ordered follow-up keeps one direction per pair through a host
 * predicate over the rendered persons.
 */
import {
  type Query,
  qlist,
  qpair,
  qtext,
  queryAtom,
  qvar,
  type Rule,
  rule,
} from "../../packages/ch4/src/04-logic.js";
import { format as formatValue } from "../../packages/ch4/src/read.js";
import type { Value } from "../../packages/ch4/src/runtime/value.js";
import { answerLines, microshaftDatabase } from "./ex_4_55.js";
import { sameRule } from "./ex_4_57.js";

/** Two people live near each other when they share a town. */
export const livesNearRule: Rule = rule(
  queryAtom("lives-near", qvar("person-1"), qvar("person-2")),
  {
    tag: "and",
    clauses: [
      queryAtom("address", qvar("person-1"), qpair(qvar("town"), qvar("rest-1"))),
      queryAtom("address", qvar("person-2"), qpair(qvar("town"), qvar("rest-2"))),
      {
        tag: "not",
        clause: queryAtom("same", qvar("person-1"), qvar("person-2")),
      },
    ],
  },
);

/** Everyone who lives near Alyssa. */
export const nearAlyssa: Query = queryAtom(
  "lives-near",
  qvar("person"),
  qlist(qtext("Hacker"), qtext("Alyssa"), qtext("P")),
);

/** Every directed near pair. */
export const allNearPairs: Query = queryAtom("lives-near", qvar("person-1"), qvar("person-2"));

/** True when the first rendered person sorts strictly before the second. */
export const inNameOrder = (args: ReadonlyArray<Value>): boolean => {
  if (args.length !== 2) {
    return false;
  }
  const [first, second] = [args[0], args[1]];
  if (first === undefined || second === undefined) {
    return false;
  }
  return formatValue(first) < formatValue(second);
};

/** Each unordered near pair listed once, via the name-order filter. */
export const dedupedNearPairs: Query = {
  tag: "and",
  clauses: [
    queryAtom("lives-near", qvar("person-1"), qvar("person-2")),
    {
      tag: "lisp-value",
      predicate: inNameOrder,
      args: [qvar("person-1"), qvar("person-2")],
    },
  ],
};

/** The answer lines for all three near queries, in evaluator order. */
export const nearAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const db = microshaftDatabase();
  db.addRule(sameRule);
  db.addRule(livesNearRule);
  return [
    answerLines(db, nearAlyssa),
    answerLines(db, allNearPairs),
    answerLines(db, dedupedNearPairs),
  ];
};

export function ex_4_60(): string {
  const [alyssa, pairs, deduped] = nearAnswers();
  return (
    `Alyssa lives near ${alyssa.length} person; the open query lists ` +
    `${pairs.length} directed pairs, ${deduped.length} once deduped by name order.`
  );
}
