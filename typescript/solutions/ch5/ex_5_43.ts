// SPDX-License-Identifier: GPL-3.0-only
import {
  type CompilerConfig,
  compileAndGo,
  compileBlock,
  defaultConfig,
  newState,
  statementsText,
} from "../../packages/ch5/src/05-compilation.js";

const PROGRAM = "(define (sum) (define a 1) (define b 2) (+ a b))";

/** Exercise 5.43: with scan-out on, the internal defines become
 * *unassigned* let bindings followed by assignments, so the body
 * compiles exactly one define-variable! and the procedure works. */
export const ex_5_43 = (): readonly string[] => {
  const plain = compileBlock(defaultConfig(), newState(), PROGRAM);
  const scannedCfg: CompilerConfig = { ...defaultConfig(), scanOut: true };
  const scanned = compileBlock(scannedCfg, newState(), PROGRAM);
  const plainText = statementsText({ needs: [], modifies: [], stmts: plain.lines });
  const scannedText = statementsText({ needs: [], modifies: [], stmts: scanned.lines });
  if (!plainText.includes("define-variable!")) throw new Error("the plain body lacks a define");
  if (!scannedText.includes("*unassigned*")) throw new Error("the scanned body lacks the bindings");
  if (scannedText.split("(op define-variable!)").length - 1 !== 1)
    throw new Error("the scanned body keeps more than one define-variable!");
  const evaluator = compileAndGo(scannedCfg, newState(), PROGRAM, "(sum)");
  evaluator.run();
  if (!evaluator.transcript.includes("3"))
    throw new Error(`the scanned procedure computed ${evaluator.transcript.join(" ")}`);
  return [
    "plain body: internal defines compile through define-variable! when the definition executes",
    "scanned body: *unassigned* bindings plus set! forms hoisted into one let, no internal define operation left",
    `scanned session: ${evaluator.transcript.join(" ")}`,
  ];
};
