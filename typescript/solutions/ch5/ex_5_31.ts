// SPDX-License-Identifier: GPL-3.0-only
import {
  compileProgram,
  defaultConfig,
  LinkageNext,
  newState,
  statementsText,
} from "../../packages/ch5/src/05-compilation.js";

/** The saves and restores of one compilation, in emission order. */
const saves = (source: string): readonly string[] =>
  statementsText(compileProgram(defaultConfig(), newState(), source, LinkageNext))
    .split("\n")
    .filter((line) => line.startsWith("(save ") || line.startsWith("(restore "));

const CASES = ["(f 'x 'y)", "((f) 'x 'y)", "(f (g 'x) y)", "(f (g 'x) 'y)"] as const;

const EXPECTED_COUNTS = [0, 0, 4, 4] as const;

/** The four code-generated answers: quoted operands need nothing, so
 * the first two combinations keep no saves at all; the last two keep
 * the proc and argl pairs around the inner call, because the call it
 * compiles modifies both while they stay live. */
export const ex_5_31 = (): readonly string[] =>
  CASES.map((source, index) => {
    const instructions = saves(source);
    if (instructions.length !== EXPECTED_COUNTS[index]) {
      throw new Error(`${source}: expected ${EXPECTED_COUNTS[index]} saves`);
    }
    return `${source}: ${instructions.join(" ")}`;
  });
