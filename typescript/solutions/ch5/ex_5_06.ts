// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  constant,
  gotoLabel,
  gotoRegister,
  labelRef,
  type MachineStatement,
  op,
  register,
  restore,
  save,
  test,
} from "../../packages/ch5/src/01-register-machines.ts";
import { fibonacciRecursiveController, type HandTrace, runHandTrace } from "./ex_5_05.ts";
import { mark } from "./ex_5_07.ts";

/** The Fibonacci machine with the redundant save and restore removed:
 * afterfib-n-1 restores n but not continue, because the second call's
 * continue is assigned, never overwritten across the save. The answer
 * and the maximum depth are unchanged; one save/restore pair per
 * activation disappears. */
export const fibonacciReducedController: readonly MachineStatement[] = [
  assign("continue", labelRef("fib-done")),
  mark("fib-loop"),
  test("<", register("n"), constant(2)),
  branch("immediate-answer"),
  save("continue"),
  assign("continue", labelRef("afterfib-n-1")),
  save("n"),
  assign("n", op("-", register("n"), constant(1))),
  gotoLabel("fib-loop"),
  mark("afterfib-n-1"),
  restore("n"),
  assign("n", op("-", register("n"), constant(2))),
  assign("continue", labelRef("afterfib-n-2")),
  save("val"),
  gotoLabel("fib-loop"),
  mark("afterfib-n-2"),
  assign("n", register("val")),
  restore("val"),
  restore("continue"),
  assign("val", op("+", register("val"), register("n"))),
  gotoRegister("continue"),
  mark("immediate-answer"),
  assign("val", register("n")),
  gotoRegister("continue"),
  mark("fib-done"),
];

/** Compares the original and the reduced Fibonacci machines: the answer
 * and the maximum depth survive, the instruction count and the stack
 * traffic drop by the removed pair on every activation that reaches the
 * second call. */
export const ex_5_06 = (n: number): { original: HandTrace; reduced: HandTrace } => ({
  original: runHandTrace(fibonacciRecursiveController, ["n", "val", "continue"], n),
  reduced: runHandTrace(fibonacciReducedController, ["n", "val", "continue"], n),
});
