// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  assign,
  branch,
  type ControllerLine,
  c,
  getRegisterContents,
  jump,
  jumpReg,
  type Machine,
  type MachineError,
  makeMachine,
  mark,
  type Outcome,
  op,
  reg,
  renderMachineError,
  restore,
  save,
  setRegisterContents,
  test,
  type Value,
} from "../../packages/ch5/src/02-simulator.js";

/** Unwraps a machine outcome: a fault escapes with its rendered message,
 * so successful pins stay honest and error tests catch the typed error. */
export const expectOk = <A>(outcome: Outcome<A>): A => {
  if (!outcome.ok) throw new Error(renderMachineError(outcome.error));
  return outcome.value;
};

export const expectError = <A>(outcome: Outcome<A>): MachineError => {
  if (outcome.ok) throw new Error("expected a machine error");
  return outcome.error;
};

/** The book's gcd machine of 5.2: registers a, b, t, the section's
 * arithmetic operations, and the controller of 5.1.1's reduction step. */
export const gcdController: ControllerLine[] = [
  mark("test-b"),
  test("=", reg("b"), c(0)),
  branch("gcd-done"),
  assign("t", op("rem", reg("a"), reg("b"))),
  assign("a", reg("b")),
  assign("b", reg("t")),
  jump("test-b"),
  mark("gcd-done"),
];

/** A fresh gcd machine, assembled and ready for inputs. */
export const gcdMachine = (): Machine =>
  expectOk(makeMachine(["a", "b", "t"], arithmeticOperations, gcdController));

/** The recursive-exponent machine of exercise 5.4: continue is saved
 * before the subproblem and restored after the multiplication, one saved
 * continue per level; the subproblem clobbers n but never b. */
export const exptRecursiveController: ControllerLine[] = [
  assign("continue", { tag: "label", name: "expt-done" }),
  mark("expt-loop"),
  test("=", reg("n"), c(0)),
  branch("base-expt"),
  save("continue"),
  assign("continue", { tag: "label", name: "after-expt" }),
  assign("n", op("-", reg("n"), c(1))),
  jump("expt-loop"),
  mark("after-expt"),
  assign("val", op("*", reg("b"), reg("val"))),
  restore("continue"),
  jumpReg("continue"),
  mark("base-expt"),
  assign("val", c(1)),
  jumpReg("continue"),
  mark("expt-done"),
];

/** The iterative-exponent machine of exercise 5.4: counter and product,
 * no stack and no continue. */
export const exptIterativeController: ControllerLine[] = [
  assign("counter", reg("n")),
  assign("product", c(1)),
  mark("expt-iter"),
  test("=", reg("counter"), c(0)),
  branch("expt-done"),
  assign("product", op("*", reg("b"), reg("product"))),
  assign("counter", op("-", reg("counter"), c(1))),
  jump("expt-iter"),
  mark("expt-done"),
];

/** Runs one expt machine with inputs b and n, answering the named result
 * register; every run starts from a fresh machine. */
const runExpt = (controller: ControllerLine[], b: number, n: number, answerReg: string): Value => {
  const machine = expectOk(
    makeMachine(
      ["b", "n", "continue", "val", "counter", "product"],
      arithmeticOperations,
      controller,
    ),
  );
  expectOk(setRegisterContents(machine, "b", b));
  expectOk(setRegisterContents(machine, "n", n));
  expectOk(machine.start());
  return expectOk(getRegisterContents(machine, answerReg));
};

/** The host oracle: b to the n by repeated multiplication, the same
 * reduction the machines execute. */
const hostExpt = (b: number, n: number): number => {
  let product = 1;
  for (let step = 0; step < n; step += 1) product *= b;
  return product;
};

/** Both 5.4 machines on both inputs, each line pairing the machine's
 * answer with its oracle's answer. The recursive machine answers in val,
 * the iterative one in product. */
export const simulatedExptRuns = (): string[] =>
  (
    [
      ["recursive", exptRecursiveController, "val"],
      ["iterative", exptIterativeController, "product"],
    ] as const
  ).flatMap(([kind, controller, answerReg]) =>
    (
      [
        [2, 10],
        [3, 5],
      ] as const
    ).map(([b, n]) => {
      const answer = runExpt(controller, b, n, answerReg);
      return `${kind} expt(${b}, ${n}) = ${answer} (host ${hostExpt(b, n)})`;
    }),
  );

/** The gcd machine's answer for one input pair. */
export const runGcd = (a: number, b: number): Value => {
  const machine = gcdMachine();
  expectOk(setRegisterContents(machine, "a", a));
  expectOk(setRegisterContents(machine, "b", b));
  expectOk(machine.start());
  return expectOk(getRegisterContents(machine, "a"));
};
