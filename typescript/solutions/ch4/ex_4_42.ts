// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.42: the Liars puzzle. `exactly-one` requires the pair of
 * statements each girl made to disagree: one true claim and one false
 * one. The five positions are chosen with the bounded generator under
 * a distinctness requirement, and each girl's pair of claims becomes
 * one requirement of exactly-one.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

/** The puzzle program: exactly-one over each girl's pair of claims. */
export const liarsSource = `
const anIntegerBetween = (low: number, high: number): number => {
  require(low <= high);
  return choose(low, anIntegerBetween(low + 1, high));
};
const liars = (): Record<string, number> => {
  const betty = anIntegerBetween(1, 5);
  const ethel = anIntegerBetween(1, 5);
  const joan = anIntegerBetween(1, 5);
  const kitty = anIntegerBetween(1, 5);
  const mary = anIntegerBetween(1, 5);
  require(
    betty !== ethel &&
      betty !== joan &&
      betty !== kitty &&
      betty !== mary &&
      ethel !== joan &&
      ethel !== kitty &&
      ethel !== mary &&
      joan !== kitty &&
      joan !== mary &&
      kitty !== mary,
  );
  require(kitty === 2 ? betty !== 3 : betty === 3);
  require(ethel === 1 ? joan !== 2 : joan === 2);
  require(joan === 3 ? ethel !== 5 : ethel === 5);
  require(kitty === 2 ? mary !== 4 : mary === 4);
  require(mary === 4 ? betty !== 1 : betty === 1);
  return { betty, ethel, joan, kitty, mary };
};
liars();
`;

/** The unique assignment in search order. */
export const solutions = (): ReadonlyArray<string> =>
  runAmbAnswers(liarsSource, "amb-depth-first-experiment", 1).answers.map((value) => format(value));

export function ex_4_42(): string {
  return (
    "exactly-one requires the pair of statements each girl made to disagree: one claim " +
    "true, the other false. The five positions are chosen under a distinctness " +
    "requirement, and each girl's pair of claims becomes one exactly-one requirement. " +
    "The first answer is { betty: 3, ethel: 5, joan: 2, kitty: 1, mary: 4 } — Kitty " +
    "first, Joan second, Betty third, Mary fourth, Ethel fifth — and the search exhausts " +
    "after it: the assignment is the unique one."
  );
}
