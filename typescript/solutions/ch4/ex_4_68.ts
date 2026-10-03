// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.68: reverse through append-to-form. The append rules
 * are the book's own atoms: empty appends to anything as itself,
 * and a head conses on after the tail appends. Reverse strips the
 * head, reverses the tail, and appends the head at the end. The
 * forward query terminates; the backward one diverges with the naive
 * rules, because the body recurses on an unbound tail before append
 * can constrain it — so it is exhibited but never run. Addition
 * 4.68a reads palindromes off reverse: a list that reverses to
 * itself.
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

/** Append: empty is identity; heads cons on after tails append. */
export const appendRules: ReadonlyArray<Rule> = [
  rule(queryAtom("append-to-form", { tag: "nil" }, qvar("y"), qvar("y"))),
  rule(
    queryAtom(
      "append-to-form",
      qpair(qvar("u"), qvar("v")),
      qvar("y"),
      qpair(qvar("u"), qvar("z")),
    ),
    queryAtom("append-to-form", qvar("v"), qvar("y"), qvar("z")),
  ),
];

/** Reverse through the append rules. */
export const reverseRules: ReadonlyArray<Rule> = [
  rule(queryAtom("reverse", { tag: "nil" }, { tag: "nil" })),
  rule(queryAtom("reverse", qpair(qvar("u"), qvar("v")), qvar("x")), {
    tag: "and",
    clauses: [
      queryAtom("reverse", qvar("v"), qvar("w")),
      queryAtom("append-to-form", qvar("w"), qpair(qvar("u"), { tag: "nil" }), qvar("x")),
    ],
  }),
];

/** Addition 4.68a: palindromes reverse to themselves. */
export const palindromeRule: Rule = rule(
  queryAtom("palindrome", qvar("x")),
  queryAtom("reverse", qvar("x"), qvar("x")),
);

const oneTwoThree = qlist(qtext("1"), qtext("2"), qtext("3"));

/** Reverse of (1 2 3). */
export const reverseForward: Query = queryAtom("reverse", oneTwoThree, qvar("x"));

/**
 * Which list reverses to (1 2 3): the naive rules diverge here. The
 * body recurses on an unbound tail before append can constrain it,
 * so evaluation generates ever-longer pre-images instead of testing
 * the ground result. Exhibited, never executed (UNRUN).
 */
export const reverseBackward: Query = queryAtom("reverse", qvar("x"), oneTwoThree);

/** The palindrome and non-palindrome probes. */
export const palindromeYes: Query = queryAtom(
  "palindrome",
  qlist(qtext("1"), qtext("2"), qtext("1")),
);
export const palindromeNo: Query = queryAtom(
  "palindrome",
  qlist(qtext("1"), qtext("2"), qtext("3")),
);

/** The answer lines for the terminating list queries, in order. */
export const reverseAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const db: Database = makeDatabase();
  for (const candidate of [...appendRules, ...reverseRules, palindromeRule]) {
    db.addRule(candidate);
  }
  return [
    answerLines(db, reverseForward),
    answerLines(db, palindromeYes),
    answerLines(db, palindromeNo),
  ];
};

export function ex_4_68(): string {
  const [forward, yes, no] = reverseAnswers();
  return (
    `Reverse answers ${forward.length} forward query; the backward query ` +
    `diverges under the naive rules. Palindrome holds ${yes.length} of the ` +
    `probes and fails ${no.length}.`
  );
}
