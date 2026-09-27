// SPDX-License-Identifier: GPL-3.0-only
import {
  type CompilerConfig,
  compileAndGo,
  compileProgram,
  defaultConfig,
  LinkageNext,
  newState,
  statementsText,
} from "../../packages/ch5/src/05-compilation.js";

const FACTORIAL = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

const withPreserving = (on: boolean): CompilerConfig => ({ ...defaultConfig(), preservingOn: on });

const savedCount = (source: string, cfg: CompilerConfig): number =>
  statementsText(compileProgram(cfg, newState(), source, LinkageNext))
    .split("\n")
    .filter((line) => line.startsWith("(save ") || line.startsWith("(restore ")).length;

/** With the preserving mechanism off, every register the compiler
 * might need is saved blindly, so the factorial compilation stacks
 * far more pushes; the code still runs, only wastefully. */
export const ex_5_37 = (): readonly string[] => {
  const on = savedCount(FACTORIAL, withPreserving(true));
  const off = savedCount(FACTORIAL, withPreserving(false));
  if (off <= on) throw new Error("preserving off saved nothing extra");
  const evaluator = compileAndGo(withPreserving(false), newState(), FACTORIAL, "(factorial 5)");
  evaluator.run();
  const value = evaluator.transcript.at(-2);
  if (value !== "120") throw new Error(`the blind code computed ${value}`);
  const pushes = evaluator.stackStatistics();
  const reference = compileAndGo(withPreserving(true), newState(), FACTORIAL, "(factorial 5)");
  reference.run();
  const frugal = reference.stackStatistics();
  return [
    `preserving on: ${on} saves and restores in the factorial compilation`,
    `preserving off: ${off} saves and restores in the same compilation`,
    `session pushes with preserving off: ${pushes.pushes}, with preserving on: ${frugal.pushes}`,
    "both answer 120; the blind code wastes stack, the book's point about preserving being a space optimization, not a correctness requirement",
  ];
};
