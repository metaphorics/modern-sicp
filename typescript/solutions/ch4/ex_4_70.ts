// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Database } from "../../packages/ch4/src/04-logic.js";
/**
 * Exercise 4.70: assertion storage without lazy cycles. The book's
 * design binds the assertion stream so the new assertion is indexed
 * first and the stream can name itself without forcing itself
 * during construction. This engine stores assertions and rules in
 * strict keyed maps instead of streams, so there is nothing to
 * force at construction time — but the same two duties hold: every
 * added fact must be indexed by relation, and adding must never
 * traverse the store it extends.
 */
import {
  fetchAssertions,
  fetchRules,
  makeDatabase,
  qlist,
  qtext,
  queryAtom,
  qvar,
  rule,
} from "../../packages/ch4/src/04-logic.js";

/** A database with one fact and one rule added through the index. */
export const indexedDatabase = (): Database => {
  const db = makeDatabase();
  db.addAssertion(
    queryAtom(
      "job",
      qlist(qtext("Bitdiddle"), qtext("Ben")),
      qlist(qtext("computer"), qtext("wizard")),
    ),
  );
  db.addRule(rule(queryAtom("same", qvar("x"), qvar("x"))));
  return db;
};

/** Indexed retrieval counts: the fact under its relation, the rule under its. */
export const indexCounts = (): readonly [number, number] => {
  const db = indexedDatabase();
  const jobs = fetchAssertions(queryAtom("job", qvar("x"), qvar("y")), db);
  const sames = fetchRules(queryAtom("same", qvar("x"), qvar("y")), db);
  return [jobs.length, sames.length];
};

export function ex_4_70(): string {
  const [jobs, sames] = indexCounts();
  return (
    `The strict store indexes ${jobs} job fact and ${sames} identity rule ` +
    `by relation with no stream to force: indexing first and never ` +
    `traversing the store under construction are the duties the ` +
    `book's let-bindings serve, and the map design keeps both.`
  );
}
