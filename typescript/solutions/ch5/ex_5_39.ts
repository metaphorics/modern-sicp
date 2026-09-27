// SPDX-License-Identifier: GPL-3.0-only
import {
  type CompilerConfig,
  compileBlock,
  controllerReplacingDriver,
  defaultConfig,
  guardedDriver,
  makeCompiledEvaluator,
  newState,
  statementsText,
} from "../../packages/ch5/src/05-compilation.js";

/** Exercise 5.39: lexical addresses walk frame number and
 * displacement, and set! mutates the binding cell in place. The
 * machine's extend-environment binds parameters in order, so the
 * displacement indexes the frame's bindings in insertion order. */
export const ex_5_39 = (): readonly string[] => {
  const cfg: CompilerConfig = { ...defaultConfig(), lexical: true };
  const source = "(define n 10)\n((lambda (cell) (set! cell (* cell 10)) (+ n cell)) 11)";
  const { entry, lines } = compileBlock(cfg, newState(), source);
  const controller = [...controllerReplacingDriver(guardedDriver), ...lines];
  const evaluator = makeCompiledEvaluator(controller, "");
  evaluator.armEntry(entry);
  evaluator.run();
  const transcript = evaluator.transcript;
  if (!transcript.includes("120")) throw new Error(`the lexical session computed ${transcript}`);
  const instructions = statementsText({ needs: [], modifies: [], stmts: lines });
  if (!instructions.includes("lexical-address-lookup"))
    throw new Error("no lexical-address-lookup in the code");
  if (!instructions.includes("lexical-address-set!"))
    throw new Error("no lexical-address-set! in the code");
  return [
    `lexical machine session: ${transcript.join(" ")}`,
    "lexical-address-lookup detects *unassigned*; lexical-address-set! mutates the addressed value cell",
  ];
};
