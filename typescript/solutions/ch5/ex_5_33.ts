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

const ALT = "(define (factorial-alt n) (if (= n 1) 1 (* n (factorial-alt (- n 1)))))";
const BOOK = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

const summary = (source: string, cfg: CompilerConfig): { statements: number; saves: number } => {
  const lines = statementsText(compileProgram(cfg, newState(), source, LinkageNext)).split("\n");
  return {
    statements: lines.length,
    saves: lines.filter((line) => line.startsWith("(save ")).length,
  };
};

/** Exercise 5.33: the two orders compile differently because the
 * recursive operand's position changes which register is live across
 * its call; both sessions answer 120 on the compiled machine. */
export const ex_5_33 = (): readonly string[] => {
  const runCfg: CompilerConfig = { ...defaultConfig(), compoundCalls: true };
  const base = summary(BOOK, defaultConfig());
  const alt = summary(ALT, defaultConfig());
  const baseRun = compileAndGo(runCfg, newState(), BOOK, "(factorial 5)");
  baseRun.run();
  const altRun = compileAndGo(runCfg, newState(), ALT, "(factorial-alt 5)");
  altRun.run();
  if (baseRun.transcript.at(-2) !== "120") throw new Error("the book order lost 120");
  if (altRun.transcript.at(-2) !== "120") throw new Error("the alt order lost 120");
  return [
    `factorial (* (factorial (- n 1)) n): ${base.statements} statements, ${base.saves} save sites`,
    `factorial-alt (* n (factorial-alt (- n 1))): ${alt.statements} statements, ${alt.saves} save sites`,
    `factorial session: ${baseRun.transcript.join(" ")}`,
    `factorial-alt session: ${altRun.transcript.join(" ")}`,
    "the recursive operand's position changes which register is live across its call, so the save and restore pairs move with it",
  ];
};
