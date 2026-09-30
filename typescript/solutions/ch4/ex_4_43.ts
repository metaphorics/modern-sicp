// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.43: the yacht puzzle. The program chooses each father's
 * daughter, fixes Sir Barnacle's daughter as Melissa, gives the four
 * named yachts their daughters' names, draws Parker's yacht as the one
 * remaining daughter's name, forbids each father's yacht naming his
 * own daughter through the fixed namings, and requires Gabrielle's
 * father's yacht to be named after Parker's daughter. Told that Mary
 * Ann is Moore's daughter, the puzzle has one solution.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

const generators = `
const anIntegerBetween = (low: number, high: number): number => {
  require(low <= high);
  return choose(low, anIntegerBetween(low + 1, high));
};
const anElementOf = (items: string[]): string => {
  const index = anIntegerBetween(0, items.length - 1);
  const item = items[index];
  return item === undefined ? "" : item;
};
`;

/** The yacht puzzle; the flag supplies the Mary Ann Moore sentence. */
export const yachtSource = (told: boolean): string => `${generators}
const yachtSolve = (): Record<string, string> => {
  const melissa = "barnacle";
  const maryAnn = anElementOf(["moore", "downing", "hall", "barnacle", "parker"]);
  const gabrielle = anElementOf(["moore", "downing", "hall", "barnacle", "parker"]);
  const lorna = anElementOf(["moore", "downing", "hall", "barnacle", "parker"]);
  const rosalind = anElementOf(["moore", "downing", "hall", "barnacle", "parker"]);
  const parkerYacht = anElementOf(["mary-ann", "gabrielle", "lorna", "rosalind", melissa]);
  require(
    maryAnn !== gabrielle &&
      maryAnn !== lorna &&
      maryAnn !== rosalind &&
      maryAnn !== melissa &&
      gabrielle !== lorna &&
      gabrielle !== rosalind &&
      gabrielle !== melissa &&
      lorna !== rosalind &&
      lorna !== melissa &&
      rosalind !== melissa,
  );
  require(${told ? 'maryAnn === "moore"' : "true"});
  require(
    parkerYacht !== "lorna" &&
      parkerYacht !== melissa &&
      parkerYacht !== "rosalind" &&
      parkerYacht !== "gabrielle",
  );
  require(lorna !== "moore");
  require(rosalind !== "hall");
  require(gabrielle !== "barnacle");
  const parkerDaughter =
    maryAnn === "parker"
      ? "mary-ann"
      : gabrielle === "parker"
        ? "gabrielle"
        : lorna === "parker"
          ? "lorna"
          : rosalind === "parker"
            ? "rosalind"
            : "melissa";
  const yachts: Record<string, string> = {
    moore: "lorna",
    downing: melissa,
    hall: "rosalind",
    barnacle: "gabrielle",
    parker: parkerYacht,
  };
  require(yachts[gabrielle] === parkerDaughter);
  return { lornasFather: lorna };
};
yachtSolve();
`;

/** The answers: Lorna's father, in search order. */
export const solutions = (told: boolean): ReadonlyArray<string> =>
  runAmbAnswers(yachtSource(told), "amb-depth-first-experiment", 1).answers.map((value) =>
    format(value),
  );

export function ex_4_43(): string {
  return (
    "The program chooses each father's daughter, fixes Melissa as Sir Barnacle's, gives " +
    "the four named yachts their daughters' names, draws Parker's yacht as the one " +
    "remaining name, forbids each father's yacht naming his own daughter through the " +
    "fixed namings, and holds the last sentence: Gabrielle's father's yacht is named " +
    "after Parker's daughter. Told that Mary Ann is Moore's, one solution: Lorna's " +
    "father is Colonel Downing. Not told: Downing and Parker both work."
  );
}
