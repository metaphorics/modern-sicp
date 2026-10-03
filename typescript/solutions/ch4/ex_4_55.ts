// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.55: simple Microshaft queries. Three pattern queries over
 * the personnel database: everyone supervised by Ben, every accounting
 * job (dotted tail, so any title matches), and every Slumerville address
 * (dotted tail, so any street shape matches). Simple queries answer in
 * database assertion order. This module also owns the shared typed
 * Microshaft dataset the later query exercises build on.
 */
import {
  type Database,
  makeDatabase,
  type Query,
  qlist,
  qpair,
  qtext,
  queryAtom,
  queryDriverLoop,
  qvar,
} from "../../packages/ch4/src/04-logic.js";

/** The personnel database: assertions in the book's order. */
export const microshaftDatabase = (): Database => {
  const db = makeDatabase();
  const facts: ReadonlyArray<Query> = [
    queryAtom(
      "address",
      qlist(qtext("Bitdiddle"), qtext("Ben")),
      qlist(qtext("Slumerville"), qlist(qtext("Ridge"), qtext("Road")), qtext("10")),
    ),
    queryAtom(
      "job",
      qlist(qtext("Bitdiddle"), qtext("Ben")),
      qlist(qtext("computer"), qtext("wizard")),
    ),
    queryAtom("salary", qlist(qtext("Bitdiddle"), qtext("Ben")), qtext("60000")),
    queryAtom(
      "address",
      qlist(qtext("Hacker"), qtext("Alyssa"), qtext("P")),
      qlist(qtext("Cambridge"), qlist(qtext("Mass"), qtext("Ave")), qtext("78")),
    ),
    queryAtom(
      "job",
      qlist(qtext("Hacker"), qtext("Alyssa"), qtext("P")),
      qlist(qtext("computer"), qtext("programmer")),
    ),
    queryAtom("salary", qlist(qtext("Hacker"), qtext("Alyssa"), qtext("P")), qtext("40000")),
    queryAtom(
      "supervisor",
      qlist(qtext("Hacker"), qtext("Alyssa"), qtext("P")),
      qlist(qtext("Bitdiddle"), qtext("Ben")),
    ),
    queryAtom(
      "address",
      qlist(qtext("Fect"), qtext("Cy"), qtext("D")),
      qlist(qtext("Cambridge"), qlist(qtext("Ames"), qtext("Street")), qtext("3")),
    ),
    queryAtom(
      "job",
      qlist(qtext("Fect"), qtext("Cy"), qtext("D")),
      qlist(qtext("computer"), qtext("programmer")),
    ),
    queryAtom("salary", qlist(qtext("Fect"), qtext("Cy"), qtext("D")), qtext("35000")),
    queryAtom(
      "supervisor",
      qlist(qtext("Fect"), qtext("Cy"), qtext("D")),
      qlist(qtext("Bitdiddle"), qtext("Ben")),
    ),
    queryAtom(
      "address",
      qlist(qtext("Tweakit"), qtext("Lem"), qtext("E")),
      qlist(qtext("Boston"), qlist(qtext("Bay"), qtext("State"), qtext("Road")), qtext("22")),
    ),
    queryAtom(
      "job",
      qlist(qtext("Tweakit"), qtext("Lem"), qtext("E")),
      qlist(qtext("computer"), qtext("technician")),
    ),
    queryAtom("salary", qlist(qtext("Tweakit"), qtext("Lem"), qtext("E")), qtext("25000")),
    queryAtom(
      "supervisor",
      qlist(qtext("Tweakit"), qtext("Lem"), qtext("E")),
      qlist(qtext("Bitdiddle"), qtext("Ben")),
    ),
    queryAtom(
      "address",
      qlist(qtext("Reasoner"), qtext("Louis")),
      qlist(qtext("Slumerville"), qlist(qtext("Pine"), qtext("Tree"), qtext("Road")), qtext("80")),
    ),
    queryAtom(
      "job",
      qlist(qtext("Reasoner"), qtext("Louis")),
      qlist(qtext("computer"), qtext("programmer"), qtext("trainee")),
    ),
    queryAtom("salary", qlist(qtext("Reasoner"), qtext("Louis")), qtext("30000")),
    queryAtom(
      "supervisor",
      qlist(qtext("Reasoner"), qtext("Louis")),
      qlist(qtext("Hacker"), qtext("Alyssa"), qtext("P")),
    ),
    queryAtom(
      "supervisor",
      qlist(qtext("Bitdiddle"), qtext("Ben")),
      qlist(qtext("Warbucks"), qtext("Oliver")),
    ),
    queryAtom(
      "address",
      qlist(qtext("Warbucks"), qtext("Oliver")),
      qlist(qtext("Swellesley"), qlist(qtext("Top"), qtext("Heap"), qtext("Road"))),
    ),
    queryAtom(
      "job",
      qlist(qtext("Warbucks"), qtext("Oliver")),
      qlist(qtext("administration"), qtext("big"), qtext("wheel")),
    ),
    queryAtom("salary", qlist(qtext("Warbucks"), qtext("Oliver")), qtext("150000")),
    queryAtom(
      "address",
      qlist(qtext("Scrooge"), qtext("Eben")),
      qlist(qtext("Weston"), qlist(qtext("Shady"), qtext("Lane")), qtext("10")),
    ),
    queryAtom(
      "job",
      qlist(qtext("Scrooge"), qtext("Eben")),
      qlist(qtext("accounting"), qtext("chief"), qtext("accountant")),
    ),
    queryAtom("salary", qlist(qtext("Scrooge"), qtext("Eben")), qtext("75000")),
    queryAtom(
      "supervisor",
      qlist(qtext("Scrooge"), qtext("Eben")),
      qlist(qtext("Warbucks"), qtext("Oliver")),
    ),
    queryAtom(
      "address",
      qlist(qtext("Cratchet"), qtext("Robert")),
      qlist(qtext("Allston"), qlist(qtext("N"), qtext("Harvard"), qtext("Street")), qtext("16")),
    ),
    queryAtom(
      "job",
      qlist(qtext("Cratchet"), qtext("Robert")),
      qlist(qtext("accounting"), qtext("scrivener")),
    ),
    queryAtom("salary", qlist(qtext("Cratchet"), qtext("Robert")), qtext("18000")),
    queryAtom(
      "supervisor",
      qlist(qtext("Cratchet"), qtext("Robert")),
      qlist(qtext("Scrooge"), qtext("Eben")),
    ),
    queryAtom(
      "address",
      qlist(qtext("Aull"), qtext("DeWitt")),
      qlist(qtext("Slumerville"), qlist(qtext("Onion"), qtext("Square")), qtext("5")),
    ),
    queryAtom(
      "job",
      qlist(qtext("Aull"), qtext("DeWitt")),
      qlist(qtext("administration"), qtext("secretary")),
    ),
    queryAtom("salary", qlist(qtext("Aull"), qtext("DeWitt")), qtext("25000")),
    queryAtom(
      "supervisor",
      qlist(qtext("Aull"), qtext("DeWitt")),
      qlist(qtext("Warbucks"), qtext("Oliver")),
    ),
    queryAtom(
      "can-do-job",
      qlist(qtext("computer"), qtext("wizard")),
      qlist(qtext("computer"), qtext("programmer")),
    ),
    queryAtom(
      "can-do-job",
      qlist(qtext("computer"), qtext("wizard")),
      qlist(qtext("computer"), qtext("technician")),
    ),
    queryAtom(
      "can-do-job",
      qlist(qtext("computer"), qtext("programmer")),
      qlist(qtext("computer"), qtext("programmer"), qtext("trainee")),
    ),
    queryAtom(
      "can-do-job",
      qlist(qtext("administration"), qtext("secretary")),
      qlist(qtext("administration"), qtext("big"), qtext("wheel")),
    ),
  ];
  for (const fact of facts) {
    db.addAssertion(fact);
  }
  return db;
};

/**
 * The answer lines for one query: the driver transcript minus the echoed
 * query line. An empty answer set yields ["No."].
 */
export const answerLines = (db: Database, query: Query): ReadonlyArray<string> => {
  const run = queryDriverLoop(db, [query]);
  return run.transcript.slice(1);
};

/** Everyone supervised by Ben Bitdiddle. */
export const supervisedByBen: Query = queryAtom(
  "supervisor",
  qvar("person"),
  qlist(qtext("Bitdiddle"), qtext("Ben")),
);

/** Every accounting job, whatever the title. */
export const accountingJobs: Query = queryAtom(
  "job",
  qvar("person"),
  qpair(qtext("accounting"), qvar("title")),
);

/** Every Slumerville address, whatever the street shape. */
export const slumervilleAddresses: Query = queryAtom(
  "address",
  qvar("person"),
  qpair(qtext("Slumerville"), qvar("rest")),
);

/** Answers to the three database lookups, in evaluator order. */
export const simpleQueryAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const db = microshaftDatabase();
  return [
    answerLines(db, supervisedByBen),
    answerLines(db, accountingJobs),
    answerLines(db, slumervilleAddresses),
  ];
};

export function ex_4_55(): string {
  const [supervisees, accountants, residents] = simpleQueryAnswers();
  return (
    `Simple queries return ${supervisees.length} people supervised by Ben, ` +
    `${accountants.length} accounting employees, and ${residents.length} Slumerville addresses.`
  );
}
