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

const EXAMPLE =
  "(((lambda (x y) (lambda (a b c d e) ((lambda (y z) (* x y z)) (* a b x) (+ c d x)))) 3 4) 1 2 4 6 8)";

/** Exercise 5.42: bound variables compile to frame and displacement
 * constants; names absent from the compile-time environment fall back
 * to the global lookup. The nested-lambda example answers 234:
 * x=(2 0), y=(0 0), z=(0 1). */
export const ex_5_42 = (): readonly string[] => {
  const cfg: CompilerConfig = { ...defaultConfig(), lexical: true };
  const { entry, lines } = compileBlock(cfg, newState(), EXAMPLE);
  const text = statementsText({ needs: [], modifies: [], stmts: lines });
  if (!text.includes("(op lexical-address-lookup) (const 2) (const 0)"))
    throw new Error("the address of x is not (2 0)");
  if (!text.includes("(op lexical-address-lookup) (const 0) (const 1)"))
    throw new Error("the address of z is not (0 1)");
  const controller = [...controllerReplacingDriver(guardedDriver), ...lines];
  const evaluator = makeCompiledEvaluator(controller, "");
  evaluator.armEntry(entry);
  evaluator.run();
  if (!evaluator.transcript.includes("234"))
    throw new Error(`the lexical run computed ${evaluator.transcript.join(" ")}`);
  return [
    "lexical code: z=(0 1), y=(0 0), x=(2 0)",
    `nested-lambda result: ${evaluator.transcript.join(" ")}`,
  ];
};
