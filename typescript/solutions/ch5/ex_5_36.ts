// SPDX-License-Identifier: GPL-3.0-only

import type { Word } from "../../packages/ch5/src/04-eceval.js";
import {
  type CompilerConfig,
  compileBlock,
  controllerReplacingDriver,
  defaultConfig,
  guardedDriver,
  makeCompiledEvaluator,
  newState,
  type RuntimeFn,
  statementsText,
} from "../../packages/ch5/src/05-compilation.js";

const recorded: Word[] = [];

/** A primitive that answers its argument and records the order the
 * compiler evaluates operands in. */
const record: RuntimeFn = (args) => {
  recorded.push(args[0] ?? 0);
  return args[0] ?? 0;
};

const run = (leftToRight: boolean): { order: number[]; text: string } => {
  recorded.length = 0;
  const cfg: CompilerConfig = { ...defaultConfig(), leftToRight };
  const source = "(list (record 1) (record 2))";
  const { entry, lines } = compileBlock(cfg, newState(), source);
  const controller = [...controllerReplacingDriver(guardedDriver), ...lines];
  const evaluator = makeCompiledEvaluator(controller, "", {
    runtime: { record },
  });
  evaluator.armEntry(entry);
  evaluator.run();
  return {
    order: recorded.map((word) => (typeof word === "number" ? word : Number.NaN)),
    text: statementsText({ needs: [], modifies: [], stmts: lines }),
  };
};

/** The book's compiler evaluates the operands right to left, the last
 * argument initializing argl; the 5.36 left-to-right configuration
 * reverses that, and both produce the same number of instructions. */
export const ex_5_36 = (): readonly string[] => {
  const right = run(false);
  const left = run(true);
  if (right.order.join(",") !== "2,1") throw new Error(`right-to-left order: ${right.order}`);
  if (left.order.join(",") !== "1,2") throw new Error(`left-to-right order: ${left.order}`);
  if (right.text.split("\n").length !== left.text.split("\n").length)
    throw new Error("the instruction counts differ");
  return [
    `right-to-left recording order: ${JSON.stringify(right.order)}; list: (1 2)`,
    `left-to-right recording order: ${JSON.stringify(left.order)}; list: (1 2)`,
    `instruction counts: ${right.text.split("\n").length} / ${left.text.split("\n").length}`,
  ];
};
