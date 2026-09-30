// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.65: the wheel listed four times. The wheel rule joins
 * two supervisor links through a shared middle-manager; every
 * distinct middle path derives the same wheel again. Oliver Warbucks
 * has four middle paths (three through Ben, one through Scrooge) and
 * Ben has one (through Hacker to Louis), so the driver lists five
 * lines for two wheels. The deduped follow-up keeps each wheel once,
 * in first-appearance order.
 */
import { type Query, queryAtom, qvar, type Rule, rule } from "../../packages/ch4/src/04-logic.js";
import { answerLines, microshaftDatabase } from "./ex_4_55.js";

/** A wheel supervises someone who supervises someone. */
export const wheelRule: Rule = rule(queryAtom("wheel", qvar("person")), {
  tag: "and",
  clauses: [
    queryAtom("supervisor", qvar("middle-manager"), qvar("person")),
    queryAtom("supervisor", qvar("x"), qvar("middle-manager")),
  ],
});

/** Every wheel, middle paths included. */
export const allWheels: Query = queryAtom("wheel", qvar("who"));

/** The wheel lines in evaluator order, duplicates included. */
export const wheelAnswers = (): ReadonlyArray<string> => {
  const db = microshaftDatabase();
  db.addRule(wheelRule);
  return answerLines(db, allWheels);
};

/** Each wheel once, in first-appearance order. */
export const distinctWheels = (): ReadonlyArray<string> => {
  const seen = new Set<string>();
  const out: string[] = [];
  for (const line of wheelAnswers()) {
    if (!seen.has(line)) {
      seen.add(line);
      out.push(line);
    }
  }
  return out;
};

export function ex_4_65(): string {
  const answers = wheelAnswers();
  return (
    `${answers.length} wheel lines name ${distinctWheels().length} wheels: ` +
    `Warbucks rides four middle paths, Ben rides one.`
  );
}
