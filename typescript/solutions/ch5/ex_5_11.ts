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
import { fibonacciRecursiveController, type HandTrace } from "./ex_5_05.ts";
import { arithmeticOperations, expectOk, mark } from "./ex_5_07.ts";

/** Discipline (a): the name-blind pop. Saves push their register's
 * value; a restore pops whatever was saved last, whatever its register. */
export const blindOps = (): {
  operations: Readonly<Record<string, Operation>>;
  pop: () => MachineValue;
} => {
  const stack: MachineValue[] = [];
  return {
    operations: {
      "blind-push": (args) => {
        stack.push(args[0] ?? null);
        return args[0] ?? null;
      },
      "blind-pop": () => {
        const value = stack.pop();
        return value === undefined ? null : value;
      },
    },
    pop: () => {
      const value = stack.pop();
      return value === undefined ? null : value;
    },
  };
};

/** Discipline (c): one stack per register. Saves push onto the named
 * register's own stack; a restore pops only from that stack. */
export const perRegisterOps = (): Readonly<Record<string, Operation>> => {
  const stacks = new Map<string, MachineValue[]>();
  const stackOf = (name: string): MachineValue[] => {
    const found = stacks.get(name);
    if (found !== undefined) return found;
    const fresh: MachineValue[] = [];
    stacks.set(name, fresh);
    return fresh;
  };
  return {
    "named-push": (args) => {
      const name = args[0];
      const value = args[1];
      if (typeof name !== "string") throw new Error("named-push: expected a register name");
      stackOf(name).push(value ?? null);
      return value ?? null;
    },
    "named-pop": (args) => {
      const name = args[0];
      if (typeof name !== "string") throw new Error("named-pop: expected a register name");
      const value = stackOf(name).pop();
      return value === undefined ? null : value;
    },
  };
};

/** Rewrites one controller onto a discipline: (a) and (c) replace the
 * stack instructions with the discipline's operations, (b) is the
 * standard checking stack and stays as written. */
export const withDiscipline = (
  discipline: "blind" | "checking" | "per-register",
  controller: readonly MachineStatement[],
): readonly MachineStatement[] => {
  if (discipline === "checking") return controller;
  const push = discipline === "blind" ? "blind-push" : "named-push";
  const pop = discipline === "blind" ? "blind-pop" : "named-pop";
  return controller.map((statement) => {
    if (statement.tag === "save") {
      return perform(
        push,
        ...(discipline === "blind"
          ? [register(statement.register)]
          : [constant(statement.register), register(statement.register)]),
      );
    }
    if (statement.tag === "restore") {
      return assign(
        statement.register,
        op(pop, ...(discipline === "blind" ? [] : [constant(statement.register)])),
      );
    }
    return statement;
  });
};

/** The book's diagnostic sequence: save y, save x, restore y. */
export const outOfOrderController: readonly MachineStatement[] = [
  assign("y", constant(1)),
  assign("x", constant(2)),
  save("y"),
  save("x"),
  restore("y"),
];

const runSequence = (
  discipline: "blind" | "checking" | "per-register",
): { readonly value: MachineValue | null; readonly error: MachineError | null } => {
  const extra =
    discipline === "blind"
      ? blindOps().operations
      : discipline === "per-register"
        ? perRegisterOps()
        : {};
  const machine: Machine = makeMachine({
    registers: ["x", "y"],
    operations: { ...arithmeticOperations, ...extra },
    controller: withDiscipline(discipline, outOfOrderController),
  });
  const run = machine.run();
  return {
    value: run.error === null ? (machine.readRegister("y") ?? null) : null,
    error: run.error,
  };
};

const runFib = (
  discipline: "blind" | "checking" | "per-register",
  n: number,
): HandTrace | MachineError => {
  const extra =
    discipline === "blind"
      ? blindOps().operations
      : discipline === "per-register"
        ? perRegisterOps()
        : {};
  const machine = makeMachine({
    registers: ["n", "val", "continue"],
    operations: { ...arithmeticOperations, ...extra },
    controller: withDiscipline(discipline, fibonacciRecursiveController),
  });
  machine.writeRegister("n", n);
  const run = machine.run();
  if (run.error !== null) return run.error;
  const value = machine.readRegister("val");
  return {
    value: typeof value === "number" ? value : 0,
    instructionCount: run.instructionCount,
    maxDepth: run.stackStats.maxDepth,
    events: [],
  };
};

/** The part-(a) elimination: afterfib-n-2's `(assign n (reg val))`,
 * `(restore val)` collapses to one `(restore n)`. */
export const collapsedFibController: readonly MachineStatement[] = [
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
  restore("continue"),
  assign("n", op("-", register("n"), constant(2))),
  save("continue"),
  assign("continue", labelRef("afterfib-n-2")),
  save("val"),
  gotoLabel("fib-loop"),
  mark("afterfib-n-2"),
  restore("n"),
  restore("continue"),
  assign("val", op("+", register("val"), register("n"))),
  gotoRegister("continue"),
  mark("immediate-answer"),
  assign("val", register("n")),
  gotoRegister("continue"),
  mark("fib-done"),
];

const runCollapsed = (
  discipline: "blind" | "checking",
  n: number,
): { readonly value: MachineValue | null; readonly error: MachineError | null } => {
  const extra = discipline === "blind" ? blindOps().operations : {};
  const machine = makeMachine({
    registers: ["n", "val", "continue"],
    operations: { ...arithmeticOperations, ...extra },
    controller: withDiscipline(discipline, collapsedFibController),
  });
  machine.writeRegister("n", n);
  const run = machine.run();
  return {
    value: run.error === null ? (machine.readRegister("val") ?? null) : null,
    error: run.error,
  };
};

/** Exercise 5.11: the three readings of `restore` over one controller.
 * The disciplines agree on the Fibonacci machine and disagree exactly on
 * the sequence the book distinguishes them by. */
export const ex_5_11 = (): {
  readonly blind: MachineValue | null;
  readonly checking: MachineError | null;
  readonly perRegister: MachineValue | null;
  readonly fib: { readonly blind: number; readonly checking: number; readonly perRegister: number };
  readonly collapsed: {
    readonly blind: readonly MachineValue[];
    readonly checking: MachineError | null;
  };
} => {
  const sequence = {
    blind: runSequence("blind"),
    checking: runSequence("checking"),
    perRegister: runSequence("per-register"),
  };
  const fib = {
    blind: runFib("blind", 3),
    checking: runFib("checking", 3),
    perRegister: runFib("per-register", 3),
  };
  const collapsedBlind3 = runCollapsed("blind", 3);
  const collapsedBlind5 = runCollapsed("blind", 5);
  return {
    blind: sequence.blind.value,
    checking: sequence.checking.error,
    perRegister: sequence.perRegister.value,
    fib: {
      blind: "value" in fib.blind ? fib.blind.value : 0,
      checking: "value" in fib.checking ? fib.checking.value : 0,
      perRegister: "value" in fib.perRegister ? fib.perRegister.value : 0,
    },
    collapsed: {
      blind: [collapsedBlind3.value, collapsedBlind5.value] as readonly MachineValue[],
      checking: runCollapsed("checking", 3).error,
    },
  };
};
