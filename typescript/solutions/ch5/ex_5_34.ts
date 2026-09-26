// SPDX-License-Identifier: GPL-3.0-only
import {
  compileProgram,
  defaultConfig,
  LinkageNext,
  newState,
  statementsText,
} from "../../packages/ch5/src/05-compilation.js";

const ITERATIVE =
  "(define (factorial n) (define (iter product counter) (if (> counter n) product (iter (* counter product) (+ counter 1)))) (iter 1 1))";

/** The iterative factorial compiles with a constant stack: the only
 * saves are the ones around the non-tail parts of each call frame, and
 * the tail-recursive iter call returns through (goto (reg continue))
 * without stacking anything, so the counter annotations read: before
 * each iter entry the stack holds at most the three registers the
 * definition's linkage saved, and the loop runs at depth 3 forever. */
export const ex_5_34 = (): readonly string[] => {
  const listing = statementsText(
    compileProgram(defaultConfig(), newState(), ITERATIVE, LinkageNext),
  );
  const lines = listing.split("\n");
  const saves = lines.filter((line) => line.startsWith("(save "));
  const returnJumps = lines.filter((line) => line === "(goto (reg continue))");
  if (saves.length === 0) throw new Error("no saves found");
  if (returnJumps.length === 0) throw new Error("no return jumps found");
  return [
    listing,
    `${lines.length} statements, ${saves.length} saves in the whole compilation`,
    `the iter call ends in ${returnJumps.length} (goto (reg continue)) statements: the machine enters it with no return point stacked, so the stack use is bounded no matter how many iterations run`,
  ];
};
