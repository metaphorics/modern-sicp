// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.40: pruning before the restrictions. The counting
 * identities: before the distinctness requirement there are 5^5 =
 * 3125 sets of assignments; after it, 5! = 120. The pruned procedure
 * also moves each restriction in front of the choices it does not
 * mention: Cooper draws from the four floors the bottom-floor rule
 * allows, Fletcher from the three neither top nor bottom, Miller draws
 * above Cooper directly through the bounded generator, and the
 * adjacency and distinctness requirements run as soon as the people
 * they mention have drawn. The hint's nest of bindings is exactly this
 * shape.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

const generator = `
const anIntegerBetween = (low: number, high: number): number => {
  require(low <= high);
  return choose(low, anIntegerBetween(low + 1, high));
};
`;

/** The naive order: every choice first, every requirement after. */
export const naiveSource = `${generator}
const dwelling = (): Record<string, number> => {
  const baker = anIntegerBetween(1, 5);
  const cooper = anIntegerBetween(1, 5);
  const fletcher = anIntegerBetween(1, 5);
  const miller = anIntegerBetween(1, 5);
  const smith = anIntegerBetween(1, 5);
  const distinct =
    baker !== cooper &&
    baker !== fletcher &&
    baker !== miller &&
    baker !== smith &&
    cooper !== fletcher &&
    cooper !== miller &&
    cooper !== smith &&
    fletcher !== miller &&
    fletcher !== smith &&
    miller !== smith;
  require(distinct);
  require(baker !== 5);
  require(cooper !== 1);
  require(fletcher !== 5);
  require(fletcher !== 1);
  require(miller > cooper);
  require(Math.abs(smith - fletcher) !== 1);
  require(Math.abs(fletcher - cooper) !== 1);
  return { baker, cooper, fletcher, miller, smith };
};
dwelling();
`;

/** The pruned order: each restriction before the choices it does not mention. */
export const prunedSource = `${generator}
const dwelling = (): Record<string, number> => {
  const cooper = anIntegerBetween(2, 5);
  const fletcher = anIntegerBetween(2, 4);
  require(Math.abs(fletcher - cooper) !== 1);
  const miller = anIntegerBetween(cooper + 1, 5);
  const baker = anIntegerBetween(1, 4);
  const smith = anIntegerBetween(1, 5);
  require(Math.abs(smith - fletcher) !== 1);
  require(
    baker !== cooper &&
      baker !== fletcher &&
      baker !== miller &&
      baker !== smith &&
      cooper !== fletcher &&
      cooper !== miller &&
      cooper !== smith &&
      fletcher !== miller &&
      fletcher !== smith &&
      miller !== smith,
  );
  return { baker, cooper, fletcher, miller, smith };
};
dwelling();
`;

/** The naive search run, including its live deferred-backtrack count. */
export const naiveRun = () => runAmbAnswers(naiveSource, "amb-depth-first-experiment", 1);

/** The pruned search run, including its live deferred-backtrack count. */
export const prunedRun = () => runAmbAnswers(prunedSource, "amb-depth-first-experiment", 1);

/** The naive order's answers. */
export const naiveSolutions = (): ReadonlyArray<string> =>
  naiveRun().answers.map((value) => format(value));

/** The pruned order's answers. */
export const prunedSolutions = (): ReadonlyArray<string> =>
  prunedRun().answers.map((value) => format(value));

/** Actual deferred-backtrack counts for the two search orders. */
export const failureCounts = (): readonly [number, number] => [
  naiveRun().failures,
  prunedRun().failures,
];

export function ex_4_40(): string {
  const [naive, pruned] = failureCounts();
  return (
    "Before the distinctness requirement there are 5^5 = 3125 sets of assignments; after " +
    "it, 5! = 120. The pruned procedure moves each restriction in front of the choices it " +
    "does not mention: Cooper draws from the four allowed floors, Fletcher from the three " +
    "neither top nor bottom, Miller draws above Cooper directly, and the adjacency and " +
    "distinctness requirements run as soon as the people they mention have drawn. Both " +
    `procedures answer the same single solution; measured deferred-backtrack counts are ${naive} ` +
    `for naive order and ${pruned} for the pruned search.`
  );
}
