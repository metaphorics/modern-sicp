// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.69: greats through grandson-ended relations. The
 * compound relationship of the book becomes a `related` atom whose
 * first field is the relationship list: `grandson` chains two son
 * links, and a `great`-headed relation holds when its tail does one
 * generation lower. Ground queries terminate; the open-relationship
 * query enumerates ever-longer relations without end, so it is
 * exhibited but never run.
 */
import {
  type Database,
  type Query,
  qlist,
  qpair,
  qtext,
  queryAtom,
  qvar,
  type Rule,
  rule,
  type Term,
} from "../../packages/ch4/src/04-logic.js";
import { answerLines } from "./ex_4_55.js";
import { familyRules, genealogyDatabase } from "./ex_4_63.js";

const grandson: Term = qlist(qtext("grandson"));
const great = (rest: Term): Term => qpair(qtext("great"), rest);

/** A relation ending in grandson: the base list, or great of one. */
export const endsInGrandsonRules: ReadonlyArray<Rule> = [
  rule(queryAtom("ends-in-grandson", grandson)),
  rule(
    queryAtom("ends-in-grandson", great(qvar("rest"))),
    queryAtom("ends-in-grandson", qvar("rest")),
  ),
];
/** Related by a list relation: grandson base, great-headed step. */
export const relatedRules: ReadonlyArray<Rule> = [
  rule(
    queryAtom("related", grandson, qvar("ancestor"), qvar("descendant")),
    queryAtom("grandson", qvar("descendant"), qvar("ancestor")),
  ),
  rule(queryAtom("related", great(qvar("rel")), qvar("x"), qvar("y")), {
    tag: "and",
    clauses: [
      queryAtom("ends-in-grandson", qvar("rel")),
      queryAtom("related", qvar("rel"), qvar("x"), qvar("z")),
      queryAtom("son", qvar("z"), qvar("y")),
    ],
  }),
];

/** Irad, the great-grandson of Adam. */
export const iradsRelation: Query = queryAtom(
  "related",
  great(grandson),
  qtext("Adam"),
  qvar("who"),
);

/** The great-great-great-great-great-grandsons of Adam. */
export const distantRelations: Query = queryAtom(
  "related",
  great(great(great(great(great(grandson))))),
  qtext("Adam"),
  qvar("who"),
);

/**
 * Which relationship links Adam to Irad: the relation enumerates
 * ever-longer grandson-ended lists, so the driver would never
 * return. Exhibited, never executed (UNRUN).
 */
export const openRelationship: Query = queryAtom(
  "related",
  qvar("relationship"),
  qtext("Adam"),
  qtext("Irad"),
);

/** The answer lines for the two ground great queries, in order. */
export const greatAnswers = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const db: Database = genealogyDatabase();
  for (const candidate of [...familyRules, ...endsInGrandsonRules, ...relatedRules]) {
    db.addRule(candidate);
  }
  return [answerLines(db, iradsRelation), answerLines(db, distantRelations)];
};

export function ex_4_69(): string {
  const [irad, distant] = greatAnswers();
  return (
    `Adam's great-grandson is ${irad.length} person; his five-greats ` +
    `grandsons are ${distant.length}. The open-relationship query would ` +
    `enumerate relations forever.`
  );
}
