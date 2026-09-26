// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";
import type { Value } from "../../packages/ch4/src/core.js";

export const compoundQueries = [
  "(and (supervisor ?person (Bitdiddle Ben)) (address ?person ?where))",
  "(and (salary ?person ?salary) (salary (Bitdiddle Ben) ?bens-salary) (lisp-value < ?salary ?bens-salary))",
  "(and (supervisor ?person ?boss) (not (job ?boss (computer . ?type))))",
] as const;

const lessThan = (args: ReadonlyArray<Value>): boolean => {
  const [left, right] = args;
  return left?._tag === "Number" && right?._tag === "Number" && left.n < right.n;
};

/** The three compound-query result streams, in query-engine order. */
export const compoundAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const engine = microshaft();
  engine.setPredicates({ "<": lessThan });
  return [
    engine.answers(compoundQueries[0]),
    engine.answers(compoundQueries[1]),
    engine.answers(compoundQueries[2]),
  ];
};

export function ex_4_56(): string {
  const [addresses, salaries, outsideComputer] = compoundAnswers();
  return `The compound queries find ${addresses.length} supervisee addresses, ${salaries.length} lower-paid people, and ${outsideComputer.length} employees with a non-computer-division supervisor.`;
}
