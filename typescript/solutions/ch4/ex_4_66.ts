// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.66: accumulation over frames. Ben's plan feeds a query
 * pattern to the evaluator, extracts one variable from every frame,
 * and reduces. It breaks on derivations like the wheel's: one wheel
 * with four middle paths accumulates four times. The salvage
 * deduplicates by instantiated answer before reducing. The sum half
 * matches ground salary facts with the exported matcher and reads
 * amounts out of frames; the duplicate half shows through the
 * driver's wheel lines.
 */
import {
  bindingInFrame,
  formatTerm,
  patternMatch,
  qlist,
  qtext,
  qvar,
  type Term,
} from "../../packages/ch4/src/04-logic.js";
import { answerLines, microshaftDatabase } from "./ex_4_55.js";
import { allWheels, distinctWheels, wheelAnswers, wheelRule } from "./ex_4_65.js";

/** The numeric value of a salary text term, if it holds one. */
export const salaryNumber = (term: Term): number | undefined =>
  term.tag === "text" && typeof term.value === "string" ? Number(term.value) : undefined;

/**
 * The salaries of everyone whose job matches the given job pattern:
 * each job fact is pattern-matched, the bound person keys the salary
 * facts, and the bound amount is read back out of its frame.
 */
export const matchingSalaries = (jobPattern: Term): number[] => {
  const db = microshaftDatabase();
  const salaries = db.assertions.get("salary") ?? [];
  const jobs = db.assertions.get("job") ?? [];
  const out: number[] = [];
  for (const jobFact of jobs) {
    if (jobFact.tag !== "atom") {
      continue;
    }
    const personTerm = jobFact.fields[0];
    const jobTerm = jobFact.fields[1];
    if (personTerm === undefined || jobTerm === undefined) {
      continue;
    }
    if (patternMatch(jobPattern, jobTerm, []) === undefined) {
      continue;
    }
    const personKey = formatTerm(personTerm);
    for (const salaryFact of salaries) {
      if (salaryFact.tag !== "atom") {
        continue;
      }
      const salaryPerson = salaryFact.fields[0];
      const amountTerm = salaryFact.fields[1];
      if (salaryPerson === undefined || amountTerm === undefined) {
        continue;
      }
      if (formatTerm(salaryPerson) !== personKey) {
        continue;
      }
      const frame = patternMatch(qvar("amount"), amountTerm, []);
      if (frame === undefined) {
        continue;
      }
      const bound = bindingInFrame("amount", frame);
      if (bound === undefined) {
        continue;
      }
      const amount = salaryNumber(bound);
      if (amount !== undefined) {
        out.push(amount);
      }
    }
  }
  return out;
};

/** The computer programmers' salaries, in database order. */
export const programmerSalaries = (): number[] =>
  matchingSalaries(qlist(qtext("computer"), qtext("programmer")));

/** Their total: the accumulation Ben wanted. */
export const programmerSalarySum = (): number =>
  programmerSalaries().reduce((total, amount) => total + amount, 0);

/** The naive wheel census: one count per derivation, duplicates in. */
export const naiveWheelCount = (): number => wheelAnswers().length;

/** The salvaged census: deduplicated by instantiated answer first. */
export const salvagedWheelCount = (): number => distinctWheels().length;

/** The wheel lines the salvage deduplicates. */
export const wheelLinesForAccumulation = (): ReadonlyArray<string> => {
  const db = microshaftDatabase();
  db.addRule(wheelRule);
  return answerLines(db, allWheels);
};

export function ex_4_66(): string {
  return (
    `Programmer salaries total ${programmerSalarySum()}; the naive wheel ` +
    `census counts ${naiveWheelCount()}, the salvaged one ${salvagedWheelCount()}. ` +
    `Ben's realization: frames count derivations, not distinct answers.`
  );
}
