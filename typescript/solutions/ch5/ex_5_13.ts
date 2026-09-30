// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MachineStatement } from "../../packages/ch5/src/01-register-machines.ts";
import { makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, expectOk, gcdController } from "./ex_5_07.ts";

/** Exercise 5.13: the controller decides the registers. The scan names
 * every assign target, every register source or operand, every goto
 * register, and every save and restore register, the same walk the 5.12
 * summary uses for its registers line. */
export const controllerRegisters = (controller: readonly MachineStatement[]): readonly string[] => {
  const names = new Set<string>();
  const add = (name: string): void => {
    names.add(name);
  };
  for (const statement of controller) {
    if (statement.tag === "label") continue;
    if (statement.tag === "assign") {
      add(statement.register);
      const source = statement.source;
      if (source.tag === "reg") add(source.name);
      if (source.tag === "op") {
        for (const arg of source.args) {
          if (arg.tag === "reg") add(arg.name);
        }
      }
    }
    if (statement.tag === "test" || statement.tag === "perform") {
      for (const arg of statement.args) {
        if (arg.tag === "reg") add(arg.name);
      }
    }
    if (
      statement.tag === "goto-register" ||
      statement.tag === "save" ||
      statement.tag === "restore"
    ) {
      add(statement.register);
    }
  }
  return [...names].sort();
};

/** The machine built from the derived list answers exactly as the
 * hand-listed machine. */
export const ex_5_13 = (
  a: number,
  b: number,
): {
  readonly registers: readonly string[];
  readonly answer: number | null;
} => {
  const registers = controllerRegisters(gcdController);
  const machine = makeMachine({
    registers,
    operations: arithmeticOperations,
    controller: gcdController,
  });
  machine.writeRegister("a", a);
  machine.writeRegister("b", b);
  const run = machine.run();
  expectOk(run);
  const answer = machine.readRegister("a");
  return { registers, answer: typeof answer === "number" ? answer : null };
};
