// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";

export const meetings = `
(meeting accounting (Monday 9am))
(meeting administration (Monday 10am))
(meeting computer (Wednesday 3pm))
(meeting administration (Friday 1pm))
(meeting whole-company (Wednesday 4pm))
(rule (meeting-time ?person ?day-and-time)
  (or (meeting whole-company ?day-and-time)
      (and (job ?person (?division . ?title))
           (meeting ?division ?day-and-time))))
`;

export const meetingAnswers = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const engine = microshaft();
  engine.load(meetings);
  return [
    engine.answers("(meeting ?division (Friday . ?time))"),
    engine.answers("(meeting-time (Hacker Alyssa P) (Wednesday . ?time))"),
  ];
};

export function ex_4_59(): string {
  const [friday, alyssaWednesday] = meetingAnswers();
  return `Friday meetings: ${friday.join("; ")}. Alyssa's Wednesday meetings: ${alyssaWednesday.join("; ")}.`;
}
