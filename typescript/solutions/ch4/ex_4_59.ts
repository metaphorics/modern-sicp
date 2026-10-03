// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.59: meeting time. Ben's Friday query is a day-headed
 * meeting pattern; Alyssa's meeting-time rule joins every
 * whole-company meeting with the meetings of the person's own
 * division; her Wednesday query instantiates the person and the day.
 * Addition 4.59a extracts the division leg as a two-clause rule of its
 * own and checks it agrees with the or-branch on Alyssa's Wednesday.
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
} from "../../packages/ch4/src/04-logic.js";
import { answerLines, microshaftDatabase } from "./ex_4_55.js";

/** The weekly meetings, in the book's assertion order. */
export const meetingDatabase = (): Database => {
  const db = microshaftDatabase();
  const meetings: ReadonlyArray<Query> = [
    queryAtom("meeting", qtext("accounting"), qlist(qtext("Monday"), qtext("9am"))),
    queryAtom("meeting", qtext("administration"), qlist(qtext("Monday"), qtext("10am"))),
    queryAtom("meeting", qtext("computer"), qlist(qtext("Wednesday"), qtext("3pm"))),
    queryAtom("meeting", qtext("administration"), qlist(qtext("Friday"), qtext("1pm"))),
    queryAtom("meeting", qtext("whole-company"), qlist(qtext("Wednesday"), qtext("4pm"))),
  ];
  for (const meeting of meetings) {
    db.addAssertion(meeting);
  }
  return db;
};

/** Ben's Friday query: every meeting on a Friday time. */
export const fridayMeetings: Query = queryAtom(
  "meeting",
  qvar("division"),
  qlist(qtext("Friday"), qvar("time")),
);

/** Alyssa's rule: whole-company meetings plus the person's division. */
export const meetingTimeRule: Rule = rule(
  queryAtom("meeting-time", qvar("person"), qvar("day-and-time")),
  {
    tag: "or",
    clauses: [
      queryAtom("meeting", qtext("whole-company"), qvar("day-and-time")),
      {
        tag: "and",
        clauses: [
          queryAtom("job", qvar("person"), qpair(qvar("division"), qvar("job-rest"))),
          queryAtom("meeting", qvar("division"), qvar("day-and-time")),
        ],
      },
    ],
  },
);

/** Addition 4.59a: the division leg as a two-clause rule of its own. */
export const divisionMeetingRule: Rule = rule(
  queryAtom("division-meeting-time", qvar("person"), qvar("day-and-time")),
  {
    tag: "and",
    clauses: [
      queryAtom("job", qvar("person"), qpair(qvar("division"), qvar("job-rest"))),
      queryAtom("meeting", qvar("division"), qvar("day-and-time")),
    ],
  },
);

/** Alyssa's Wednesday query: her meetings on a Wednesday time. */
export const alyssaWednesday: Query = queryAtom(
  "meeting-time",
  qlist(qtext("Hacker"), qtext("Alyssa"), qtext("P")),
  qlist(qtext("Wednesday"), qvar("time")),
);

/** The division-leg Wednesday query for the addition. */
export const alyssaDivisionWednesday: Query = queryAtom(
  "division-meeting-time",
  qlist(qtext("Hacker"), qtext("Alyssa"), qtext("P")),
  qlist(qtext("Wednesday"), qvar("time")),
);

/** The answer lines for all three meeting queries, in evaluator order. */
export const meetingAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const db = meetingDatabase();
  db.addRule(meetingTimeRule);
  db.addRule(divisionMeetingRule);
  return [
    answerLines(db, fridayMeetings),
    answerLines(db, alyssaWednesday),
    answerLines(db, alyssaDivisionWednesday),
  ];
};

export function ex_4_59(): string {
  const [friday, wednesday, division] = meetingAnswers();
  return (
    `Friday holds ${friday.length} meeting; Alyssa's Wednesday holds ` +
    `${wednesday.length}, ${division.length} of them divisional.`
  );
}
