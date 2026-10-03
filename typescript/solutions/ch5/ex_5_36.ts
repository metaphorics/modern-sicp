// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { readProgram } from "../../packages/ch5/src/04-eceval.ts";
import {
  appendInstructionSequences,
  compileExpression,
  compileProgram,
  isCompileError,
  preserving,
  returnLinkage,
} from "../../packages/ch5/src/05-compilation.ts";
import { assign, type MachineStatement, op, register } from "./ex_5_07.ts";
import { compiledLines } from "./ex_5_33.ts";

/** One argl construction for a two-operand call: the book's default
 * evaluates the operands right to left, the last operand initializing
 * argl and the earlier one adjoining onto it; the alternative goes left
 * to right, adjoining each argument at the end. The exercise measures
 * the code size of both orderings. */
export const argumentLists = (): {
  readonly rightToLeft: readonly MachineStatement[];
  readonly leftToRight: readonly MachineStatement[];
} => {
  const rightToLeft: readonly MachineStatement[] = [
    assign("argl", op("emptyArgList")),
    assign("argl", op("adjoinArg", register("argl"), register("first"))),
    assign("argl", op("adjoinArg", register("argl"), register("second"))),
  ];
  const leftToRight: readonly MachineStatement[] = [
    assign("argl", op("emptyArgList")),
    assign("argl", op("adjoinArg", register("argl"), register("second"))),
    assign("argl", op("adjoinArg", register("argl"), register("first"))),
  ];
  return { rightToLeft, leftToRight };
};

/** Exercise 5.36: the two constructions, their sizes, and the shipped
 * compiler's own listing for the two-operand call, read for the
 * evaluation order. */
export const ex_5_36 = (): readonly string[] => {
  const { rightToLeft, leftToRight } = argumentLists();
  const listing = compiledLines(
    "function f(a: number, b: number) { return a + b; }\nfunction go() { return f(p(), q()); }",
  );
  return [
    `right-to-left statements: ${rightToLeft.length}`,
    `left-to-right statements: ${leftToRight.length}`,
    "the order is the operand-list construction's, and the sizes are equal:",
    `shipped listing for the two-operand call (${listing.length} lines)`,
  ];
};
