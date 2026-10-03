// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.77: delayed filtering for `not`. Filtering against a
 * frame with unbound variables answers the wrong question: `not`
 * with an open pattern matches something and kills every frame.
 * The cure is waiting — filter only once the clause's variables are
 * bound, as soon as possible to cut intermediate frames but never
 * before the pattern is ground enough to mean it. Addition 4.77a
 * probes the middle ground: a partially bound clause still
 * over-filters, which is why the design waits for full grounding of
 * the filter pattern.
 */
import { type Query, qlist, qtext, queryAtom, qvar } from "../../packages/ch4/src/04-logic.js";
import { answerLines, microshaftDatabase } from "./ex_4_55.js";

/** Ben's supervisees filtered by a bound job pattern (sound). */
export const boundFilter: Query = {
  tag: "and",
  clauses: [
    queryAtom("supervisor", qvar("person"), qlist(qtext("Bitdiddle"), qtext("Ben"))),
    {
      tag: "not",
      clause: queryAtom("job", qvar("person"), qlist(qtext("computer"), qtext("programmer"))),
    },
  ],
};

/** The same filter with the job left open (unsound): every frame dies. */
export const openFilter: Query = {
  tag: "and",
  clauses: [
    queryAtom("supervisor", qvar("person"), qlist(qtext("Bitdiddle"), qtext("Ben"))),
    {
      tag: "not",
      clause: queryAtom("job", qvar("person"), qvar("job")),
    },
  ],
};

/**
 * Addition 4.77a: the partially bound probe. The supervisor is
 * bound but the title stays open, so the clause still matches
 * something for every frame and the filter still kills them all —
 * partial grounding is not grounding.
 */
export const partialFilter: Query = {
  tag: "and",
  clauses: [
    queryAtom("supervisor", qvar("person"), qlist(qtext("Bitdiddle"), qtext("Ben"))),
    {
      tag: "not",
      clause: queryAtom("job", qvar("person"), qlist(qtext("computer"), qvar("title"))),
    },
  ],
};

/** The answer lines for all three filters, in evaluator order. */
export const filterAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const db = microshaftDatabase();
  return [
    answerLines(db, boundFilter),
    answerLines(db, openFilter),
    answerLines(db, partialFilter),
  ];
};

export function ex_4_77(): string {
  const [bound, open, partial] = filterAnswers();
  const kept = (lines: ReadonlyArray<string>): number =>
    lines.filter((line) => line !== "No.").length;
  return (
    `The bound filter keeps ${kept(bound)} supervisee; the open filter ` +
    `keeps ${kept(open)} and the partial filter keeps ${kept(partial)}. ` +
    `Filter as soon as the pattern is ground — sooner cuts frames, ` +
    `earlier answers the wrong question.`
  );
}
