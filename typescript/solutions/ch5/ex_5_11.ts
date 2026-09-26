// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  type AssemblerOptions,
  arithmeticOperations,
  assemble,
  assign,
  branch,
  type ControllerLine,
  c,
  type Exec,
  fail,
  getRegisterContents,
  type Instruction,
  jump,
  jumpReg,
  lbl,
  type Machine,
  makeNewMachine,
  mark,
  type Outcome,
  ok,
  op,
  reg,
  renderMachineError,
  renderValue,
  restore,
  save,
  setRegisterContents,
  test,
  type Value,
} from "../../packages/ch5/src/02-simulator.js";
import { expectError, expectOk } from "./ex_5_07.js";

/** The recursive Fibonacci machine of figure 5.12, transcribed line for
 * line: each level saves continue before the first call, saves n beside
 * it, and restores the pair at afterfib-n-1, so the second call can be
 * set up over them. */
export const fibSimController: ControllerLine[] = [
  assign("continue", lbl("fib-done")),
  mark("fib-loop"),
  test("<", reg("n"), c(2)),
  branch("immediate-answer"),
  save("continue"),
  assign("continue", lbl("afterfib-n-1")),
  save("n"),
  assign("n", op("-", reg("n"), c(1))),
  jump("fib-loop"),
  mark("afterfib-n-1"),
  restore("n"),
  restore("continue"),
  assign("n", op("-", reg("n"), c(2))),
  save("continue"),
  assign("continue", lbl("afterfib-n-2")),
  save("val"),
  jump("fib-loop"),
  mark("afterfib-n-2"),
  assign("n", reg("val")),
  restore("val"),
  restore("continue"),
  assign("val", op("+", reg("val"), reg("n"))),
  jumpReg("continue"),
  mark("immediate-answer"),
  assign("val", reg("n")),
  jumpReg("continue"),
  mark("fib-done"),
];

/** The afterfib-n-2 entry of the figure, trimmed by part (a)'s
 * elimination: (restore n) pops the saved val directly, so (assign n
 * (reg val)) is gone and the final sum's operands are swapped. */
export const fibNameBlindController: ControllerLine[] = [
  ...fibSimController.slice(
    0,
    fibSimController.findIndex((line) => line.tag === "label" && line.name === "afterfib-n-2"),
  ),
  mark("afterfib-n-2"),
  restore("n"),
  restore("continue"),
  assign("val", op("+", reg("val"), reg("n"))),
  jumpReg("continue"),
  mark("immediate-answer"),
  assign("val", reg("n")),
  jumpReg("continue"),
  mark("fib-done"),
];

/** The book's out-of-order sequence: restore y after x, not y, was
 * saved last. */
const outOfOrderController: ControllerLine[] = [
  assign("y", c(1)),
  assign("x", c(2)),
  save("y"),
  save("x"),
  restore("y"),
  mark("done"),
];

export type Discipline = "a" | "b" | "c";

/** Discipline (b)'s saved cells: one word per entry, tagged with the
 * register it was saved from. */
interface SavedWord {
  readonly reg: string;
  readonly value: Value;
}

/** The tagged save: pushes the register's content with its name. */
const taggedSave = (
  instr: Extract<Instruction, { tag: "save" }>,
  machine: Machine,
  saved: SavedWord[],
): Outcome<Exec> => {
  const source = machine.registerFor(instr.reg);
  if (!source.ok) return source;
  const register = source.value;
  return ok(() => {
    saved.push({ reg: instr.reg, value: register.content() });
    machine.pc += 1;
    return ok(null);
  });
};

/** The tagged restore: pops the most recent cell, refusing a differently
 * named register with the typed mismatch. */
const taggedRestore = (
  instr: Extract<Instruction, { tag: "restore" }>,
  machine: Machine,
  saved: SavedWord[],
): Outcome<Exec> => {
  const target = machine.registerFor(instr.reg);
  if (!target.ok) return target;
  const register = target.value;
  return ok(() => {
    const entry = saved.pop();
    if (!entry) return fail({ tag: "StackUnderflow", reg: instr.reg });
    if (entry.reg !== instr.reg) {
      return fail({ tag: "RestoreMismatch", wanted: instr.reg, found: entry.reg });
    }
    register.store(entry.value);
    machine.pc += 1;
    return ok(null);
  });
};

/** The per-register save: pushes onto the register's own stack. */
const perRegisterSave = (
  instr: Extract<Instruction, { tag: "save" }>,
  machine: Machine,
  stacks: Map<string, Value[]>,
): Outcome<Exec> => {
  const source = machine.registerFor(instr.reg);
  if (!source.ok) return source;
  const register = source.value;
  return ok(() => {
    const cells = stacks.get(instr.reg) ?? [];
    cells.push(register.content());
    stacks.set(instr.reg, cells);
    machine.pc += 1;
    return ok(null);
  });
};

/** The per-register restore: pops the register's own stack. */
const perRegisterRestore = (
  instr: Extract<Instruction, { tag: "restore" }>,
  machine: Machine,
  stacks: Map<string, Value[]>,
): Outcome<Exec> => {
  const target = machine.registerFor(instr.reg);
  if (!target.ok) return target;
  const register = target.value;
  return ok(() => {
    const cells = stacks.get(instr.reg) ?? [];
    const value = cells.pop();
    if (value === undefined) return fail({ tag: "StackUnderflow", reg: instr.reg });
    register.store(value);
    machine.pc += 1;
    return ok(null);
  });
};

/** One machine with its discipline: (a) the standard simulator, (b) the
 * tagged stack, (c) one stack per register. The variant state is born
 * with the machine, and the assembler options wire the save and restore
 * builders; every other instruction uses the standard builders. */
export const makeDisciplineMachine = (
  registerNames: string[],
  discipline: Discipline,
): { machine: Machine; options: AssemblerOptions } => {
  const machine = makeNewMachine(registerNames, arithmeticOperations);
  if (discipline === "b") {
    const saved: SavedWord[] = [];
    return {
      machine,
      options: {
        saveRestore: (instr) =>
          instr.tag === "save"
            ? taggedSave(instr, machine, saved)
            : instr.tag === "restore"
              ? taggedRestore(instr, machine, saved)
              : ok(null),
      },
    };
  }
  if (discipline === "c") {
    const stacks = new Map<string, Value[]>();
    return {
      machine,
      options: {
        saveRestore: (instr) =>
          instr.tag === "save"
            ? perRegisterSave(instr, machine, stacks)
            : instr.tag === "restore"
              ? perRegisterRestore(instr, machine, stacks)
              : ok(null),
      },
    };
  }
  return { machine, options: {} };
};

/** Runs controller under discipline with n in register n, answering
 * val; every run is a fresh machine, so no stack carries over. */
const runFib = (
  controller: ReadonlyArray<ControllerLine>,
  discipline: Discipline,
  n: number,
): Value => {
  const { machine, options } = makeDisciplineMachine(["n", "continue", "val"], discipline);
  const program = assemble(controller, machine, options);
  if (!program.ok) throw new Error(renderMachineError(program.error));
  machine.install(program.value);
  expectOk(setRegisterContents(machine, "n", n));
  expectOk(machine.start());
  return expectOk(getRegisterContents(machine, "val"));
};

/** The book's out-of-order sequence under one discipline letter,
 * answering y's content or the typed error's text. */
const outOfOrderOutcome = (discipline: Discipline): string => {
  const { machine, options } = makeDisciplineMachine(["x", "y"], discipline);
  const program = assemble(outOfOrderController, machine, options);
  if (!program.ok) return renderMachineError(expectError(program));
  machine.install(program.value);
  const ran = machine.start();
  if (!ran.ok) return renderMachineError(expectError(ran));
  return `y = ${renderValue(expectOk(getRegisterContents(machine, "y")))}`;
};

/** The eliminated controller under the checking discipline: (restore n)
 * finds a value saved from val, the typed mismatch. */
const eliminatedUnderChecking = (): string => {
  const { machine, options } = makeDisciplineMachine(["n", "continue", "val"], "b");
  const program = assemble(fibNameBlindController, machine, options);
  if (!program.ok) return renderMachineError(expectError(program));
  machine.install(program.value);
  expectOk(setRegisterContents(machine, "n", 3));
  const ran = machine.start();
  if (!ran.ok) return renderMachineError(expectError(ran));
  return `val = ${renderValue(expectOk(getRegisterContents(machine, "val")))}`;
};

/** The three disciplines on the same out-of-order sequence and on the
 * same fib run, then part (a)'s elimination running on the name-blind
 * assembler and failing the checking one. */
export const restoreDisciplineRuns = (): string[] => [
  `out-of-order restore under (a) name-blind: ${outOfOrderOutcome("a")}`,
  `out-of-order restore under (b) checking: ${outOfOrderOutcome("b")}`,
  `out-of-order restore under (c) per-register: ${outOfOrderOutcome("c")}`,
  `fib(3) under (a): val = ${runFib(fibSimController, "a", 3)}`,
  `fib(3) under (b): val = ${runFib(fibSimController, "b", 3)}`,
  `fib(3) under (c): val = ${runFib(fibSimController, "c", 3)}`,
  `fib(3) with the eliminated assign, discipline (a): val = ${runFib(fibNameBlindController, "a", 3)}`,
  `fib(5) with the eliminated assign, discipline (a): val = ${runFib(fibNameBlindController, "a", 5)}`,
  `the eliminated controller under (b): ${eliminatedUnderChecking()}`,
];
