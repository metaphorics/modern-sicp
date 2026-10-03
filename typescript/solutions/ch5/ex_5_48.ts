// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  evaluatorController,
  gotoLabel,
  insertBeforeInstruction,
  type MachineStatement,
  makeEvaluator,
  type Operation,
  op,
  register,
  replaceSegment,
  test,
  type Word,
} from "../../packages/ch5/src/04-eceval.ts";
import { compileAndRun } from "../../packages/ch5/src/05-compilation.ts";
import { mark } from "./ex_5_07.ts";

type EvaluatorStatement = MachineStatement<Word>;

const isCallOf = (word: Word, name: string): boolean =>
  typeof word === "object" &&
  word !== null &&
  "tag" in word &&
  word.tag === "call" &&
  "callee" in word &&
  typeof word.callee === "object" &&
  word.callee !== null &&
  "tag" in word.callee &&
  word.callee.tag === "variable" &&
  "name" in word.callee &&
  word.callee.name === name;

/** Exercise 5.48: compile-and-run as a primitive of the evaluator. The
 * dispatch recognizes the call form before the generic application
 * path and one machine operation hands the operand source to the
 * compiler, so a program can compile and run new code at run time. */
export const makeCompileRunOperations = (): Readonly<Record<string, Operation<Word>>> => ({
  isCompileRunCall: (args) => isCallOf(args[0], "compileAndRun"),
  compileRunSource: (args) => {
    const source = args[0];
    if (typeof source !== "string") return undefined;
    const run = compileAndRun(source);
    return run.outcome.tag === "ok" ? run.outcome.value : undefined;
  },
  compileRunArg: (args) => {
    const call = args[0];
    if (typeof call !== "object" || call === null || !("tag" in call) || call.tag !== "call")
      return undefined;
    const first = call.args[0];
    return first?.kind === "item" && first.expr.tag === "string" ? first.expr.value : undefined;
  },
});

/** The spliced dispatch: the compile-and-run call form is recognized
 * ahead of the generic application test. */
export const compileRunController: readonly EvaluatorStatement[] = replaceSegment(
  insertBeforeInstruction(
    evaluatorController,
    (line) => line.tag === "test" && line.operation === "isCall",
    "the compile-and-run form is recognized first",
    [test("isCompileRunCall", register("expr")), branch("ev-compile-run")],
  ),
  "done",
  "done",
  [
    mark("ev-compile-run"),
    assign("val", op("compileRunSource", op("compileRunArg", register("expr")))),
    gotoLabel("continue-dispatch"),
  ],
);
/** The session: a program that compiles and runs new arithmetic at run
 * time, then answers with the compiled value. */
export const ex_5_48 = (): readonly string[] => {
  const program = [
    'function compileAndRun(source: string): number { throw new Error("compileAndRun must be intercepted by the controller"); }',
    'console.log(compileAndRun("1 + 2 * 3;"));',
    'console.log(compileAndRun("10 - 4;"));',
  ].join("\n");
  const result = makeEvaluator(program, makeCompileRunOperations(), compileRunController).run();
  return result.transcript;
};
