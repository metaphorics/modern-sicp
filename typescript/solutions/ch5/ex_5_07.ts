// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  constant,
  gotoLabel,
  gotoRegister,
  labelRef,
  type MachineError,
  type MachineStatement,
  type MachineValue,
  type Operation,
  op,
  perform,
  register,
  restore,
  save,
  test,
} from "../../packages/ch5/src/01-register-machines.ts";
import { type Machine, makeMachine } from "../../packages/ch5/src/02-simulator.ts";

/** A label statement: the exchange's `{ tag: "label" }` marker, written
 * as the plain record the contract admits. */
export const mark = (name: string): { readonly tag: "label"; readonly name: string } => ({
  tag: "label",
  name,
});

/** Renders a machine fault for a thrown message; diagnostics only. */
export const renderMachineError = (error: MachineError): string => JSON.stringify(error);

/** Unwraps a halted run: a fault escapes with its rendered message, so
 * successful pins stay honest and error tests catch the typed error. */
export const expectOk = (run: { readonly error: MachineError | null }): void => {
  if (run.error !== null) throw new Error(renderMachineError(run.error));
};

/** The fault of a run that must not halt cleanly. */
export const expectError = (run: { readonly error: MachineError | null }): MachineError => {
  if (run.error === null) throw new Error("expected a machine fault");
  return run.error;
};

/** Reads a register that must hold a word: the storage exercises pass
 * machine words unmodified through registers. */
export const requireWord = (value: MachineValue | undefined): MachineValue => {
  if (value === undefined) throw new Error("expected a word in the register");
  return value;
};

const numberArg = (who: string, value: MachineValue): number => {
  if (typeof value !== "number") throw new Error(`${who}: expected a number operand`);
  return value;
};

/** The section's arithmetic operations over machine words. */
export const arithmeticOperations: Readonly<Record<string, Operation>> = {
  "+": (args) => args.reduce<number>((sum, value) => sum + numberArg("+", value), 0),
  "-": (args) => {
    const first = numberArg("-", args[0]);
    return args.slice(1).reduce<number>((rest, value) => rest - numberArg("-", value), first);
  },
  "*": (args) => args.reduce<number>((product, value) => product * numberArg("*", value), 1),
  "/": (args) => numberArg("/", args[0]) / numberArg("/", args[1]),
  abs: (args) => Math.abs(numberArg("abs", args[0])),
  rem: (args) => numberArg("rem", args[0]) % numberArg("rem", args[1]),
  "=": (args) => numberArg("=", args[0]) === numberArg("=", args[1]),
  "<": (args) => numberArg("<", args[0]) < numberArg("<", args[1]),
  ">": (args) => numberArg(">", args[0]) > numberArg(">", args[1]),
};

/** The book's gcd machine of 5.2: registers a, b, t, the section's
 * arithmetic operations, and the controller of 5.1.1's reduction step. */
export const gcdController: readonly MachineStatement[] = [
  mark("test-b"),
  test("=", register("b"), constant(0)),
  branch("gcd-done"),
  assign("t", op("rem", register("a"), register("b"))),
  assign("a", register("b")),
  assign("b", register("t")),
  gotoLabel("test-b"),
  mark("gcd-done"),
];

/** A fresh gcd machine for the controller's registers. */
export const gcdMachine = (): Machine =>
  makeMachine({
    registers: ["a", "b", "t"],
    operations: arithmeticOperations,
    controller: gcdController,
  });

/** The recursive-exponent machine of exercise 5.4, transcribed from the
 * book's controller: each level saves continue and n, subproblem on
 * n - 1 with continue reassigned to after-expt, then restores the pair
 * and multiplies val by b; the subproblem clobbers n but never b. */
export const exptRecursiveController: readonly MachineStatement[] = [
  assign("continue", labelRef("expt-done")),
  mark("expt-loop"),
  test("=", register("n"), constant(0)),
  branch("base-case"),
  save("continue"),
  save("n"),
  assign("n", op("-", register("n"), constant(1))),
  assign("continue", labelRef("after-expt")),
  gotoLabel("expt-loop"),
  mark("after-expt"),
  restore("n"),
  restore("continue"),
  assign("val", op("*", register("b"), register("val"))),
  gotoRegister("continue"),
  mark("base-case"),
  assign("val", constant(1)),
  gotoRegister("continue"),
  mark("expt-done"),
];

/** The iterative-exponent machine of exercise 5.4: counter and product,
 * no stack and no continue. */
export const exptIterativeController: readonly MachineStatement[] = [
  assign("counter", register("n")),
  assign("product", constant(1)),
  mark("expt-iter"),
  test("=", register("counter"), constant(0)),
  branch("expt-done"),
  assign("product", op("*", register("b"), register("product"))),
  assign("counter", op("-", register("counter"), constant(1))),
  gotoLabel("expt-iter"),
  mark("expt-done"),
];

/** The gcd machine's answer for one input pair. */
export const runGcd = (a: number, b: number): MachineValue => {
  const machine = gcdMachine();
  machine.writeRegister("a", a);
  machine.writeRegister("b", b);
  const run = machine.run();
  expectOk(run);
  const answer = machine.readRegister("a");
  if (answer === undefined) throw new Error("gcd finished without a value in register a");
  return answer;
};
/** Runs the recursive and iterative 5.4 controllers in fresh machines and
 * compares their answer registers with the host exponentiation oracle. */
export const simulatedExptRuns = (): string[] => {
  const run = (
    controller: ReadonlyArray<MachineStatement>,
    registers: ReadonlyArray<string>,
    answerRegister: string,
    base: number,
    exponent: number,
  ): number => {
    const machine = makeMachine({ registers, operations: arithmeticOperations, controller });
    machine.writeRegister("b", base);
    machine.writeRegister("n", exponent);
    const result = machine.run();
    expectOk(result);
    const answer = machine.readRegister(answerRegister);
    if (typeof answer !== "number")
      throw new Error(`expt finished without a numeric value in ${answerRegister}`);
    return answer;
  };
  const cases = [
    [2, 10],
    [3, 5],
  ] as const;
  const recursive = cases.map(([base, exponent]) => {
    const result = run(
      exptRecursiveController,
      ["b", "n", "val", "continue"],
      "val",
      base,
      exponent,
    );
    return `recursive expt(${base}, ${exponent}) = ${result} (host ${base ** exponent})`;
  });
  const iterative = cases.map(([base, exponent]) => {
    const result = run(
      exptIterativeController,
      ["b", "n", "counter", "product"],
      "product",
      base,
      exponent,
    );
    return `iterative expt(${base}, ${exponent}) = ${result} (host ${base ** exponent})`;
  });
  return [...recursive, ...iterative];
};

export type { Machine, MachineError, MachineStatement, MachineValue, Operation };
export {
  assign,
  branch,
  constant,
  gotoLabel,
  gotoRegister,
  labelRef,
  makeMachine,
  op,
  perform,
  register,
  restore,
  save,
  test,
};
