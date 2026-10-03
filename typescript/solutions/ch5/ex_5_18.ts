// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MachineValue } from "../../packages/ch5/src/01-register-machines.ts";
import { makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, gcdController } from "./ex_5_07.ts";

/** A traced register reports `name: old -> new` on every store. The
 * landed machine has one write path and no per-register seam, so the
 * solution drives the machine one instruction at a time and diffs the
 * register file after each executed instruction: every store is one
 * assign instruction, so the diff is exactly the store log, reported
 * before the store's effect is observed to matter. */
export const makeTracedRegisterLog = (): {
  run: (a: number, b: number) => { log: readonly string[]; answer: number };
} => ({
  run: (a: number, b: number) => {
    const machine = makeMachine({
      registers: ["a", "b", "t"],
      operations: arithmeticOperations,
      controller: gcdController,
    });
    machine.writeRegister("a", a);
    machine.writeRegister("b", b);
    const names = ["a", "b", "t"];
    const before = new Map<string, MachineValue | undefined>();
    for (const name of names) {
      before.set(name, machine.readRegister(name));
    }
    const log: string[] = [];
    while (machine.singleStep()) {
      for (const name of names) {
        const value = machine.readRegister(name);
        const previous = before.get(name);
        if (value !== previous) {
          log.push(
            `${name}: ${previous === undefined ? "[unassigned]" : render(previous)} -> ${value === undefined ? "[unassigned]" : render(value)}`,
          );
          before.set(name, value);
        }
      }
    }
    const answer = machine.readRegister("a");
    return { log, answer: typeof answer === "number" ? answer : 0 };
  },
});

const render = (value: MachineValue): string =>
  typeof value === "number"
    ? String(value)
    : typeof value === "string"
      ? value
      : typeof value === "boolean"
        ? String(value)
        : value === null
          ? "null"
          : value === undefined
            ? "undefined"
            : `[${value.tag}]`;

/** Exercise 5.18 on gcd(12, 8): the register trace doubles as an
 * execution trace. */
export const ex_5_18 = (): { readonly log: readonly string[]; readonly answer: number } =>
  makeTracedRegisterLog().run(12, 8);
