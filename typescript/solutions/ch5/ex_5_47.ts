// SPDX-License-Identifier: GPL-3.0-only
import {
  type CompilerConfig,
  compileAndGo,
  compileProgram,
  defaultConfig,
  LinkageReturn,
  newState,
  statementsText,
} from "../../packages/ch5/src/05-compilation.js";

/** Exercise 5.47: the compound-call branch tests proc, sets continue
 * for the linkage, saves it on the evaluator's stack, and jumps
 * through unev to the evaluator's compound-apply, which answers in
 * val and returns through continue. */
export const ex_5_47 = (): readonly string[] => {
  const cfg: CompilerConfig = { ...defaultConfig(), compoundCalls: true };
  const compiled = "(define (f n) (g (+ n 1)))";
  const evaluator = compileAndGo(cfg, newState(), compiled, "(define (g x) (* x 2))\n(f 5)");
  evaluator.run();
  const transcript = evaluator.transcript;
  if (!transcript.includes("12"))
    throw new Error(`the compound call computed ${transcript.join(" ")}`);
  const instructions = statementsText(
    compileProgram(cfg, newState(), compiled, LinkageReturn),
  ).split("\n");
  const kept = instructions.filter(
    (line) =>
      line.includes("compound-procedure?") ||
      line.includes("compound-branch") ||
      line.includes("compound-apply") ||
      line === "(save continue)",
  );
  if (kept.length === 0) throw new Error("the controller carries no compound-call entries");
  return [
    `compound-call instructions: ${kept.slice(0, 12).join("; ")}`,
    `session: ${transcript.join(" ")}`,
  ];
};
