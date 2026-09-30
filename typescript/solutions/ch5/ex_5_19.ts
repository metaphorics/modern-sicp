// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  formatMachineStatement,
  type MachineValue,
} from "../../packages/ch5/src/01-register-machines.ts";
import type { Machine } from "../../packages/ch5/src/02-simulator.ts";
import { makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, gcdController } from "./ex_5_07.ts";

const render = (value: MachineValue | undefined): string =>
  typeof value === "number" ? String(value) : value === undefined ? "[unassigned]" : "[value]";

/** The breakpoint machine of exercise 5.19: a breakpoint names a label
 * and an n, and the machine parks just before the instruction under
 * that label executes for the n-th time, and again before each later
 * n-th arrival. Proceeding consumes the stop: the resumed arrival runs
 * and counts once, so the next stop is at a later n-th arrival. */
export const makeBreakpointMachine = (): {
  machine: Machine;
  setBreakpoint: (label: string, n: number) => void;
  cancelBreakpoint: (label: string) => void;
  cancelAllBreakpoints: () => void;
  start: () => string;
  proceed: () => string;
  parkedAt: () => string | null;
} => {
  const machine = makeMachine({
    registers: ["a", "b", "t"],
    operations: arithmeticOperations,
    controller: gcdController,
  });
  const breakpoints = new Map<string, number>();
  const arrivals = new Map<string, number>();
  let parked: string | null = null;
  const labelRenders = new Map<string, string>();
  for (const statement of gcdController) {
    if (statement.tag === "label") {
      labelRenders.set(formatMachineStatement(statement), statement.name);
    }
  }
  const stepToStop = (): string => {
    for (;;) {
      const before = machine.result().trace.length;
      const more = machine.singleStep();
      const after = machine.result().trace;
      const last = after[before];
      if (last !== undefined && labelRenders.has(last)) {
        const label = labelRenders.get(last);
        if (label !== undefined && breakpoints.has(label)) {
          const count = (arrivals.get(label) ?? 0) + 1;
          arrivals.set(label, count);
          const n = breakpoints.get(label) ?? 1;
          if (count % n === 0) {
            parked = label;
            return `break at ${label}: a = ${render(machine.readRegister("a"))}, b = ${render(machine.readRegister("b"))}`;
          }
        }
      }
      if (!more) {
        parked = null;
        const answer = machine.readRegister("a");
        return `finished: gcd(206, 40) = ${render(answer)}`;
      }
    }
  };
  return {
    machine,
    setBreakpoint: (label: string, n: number) => {
      breakpoints.set(label, n);
    },
    cancelBreakpoint: (label: string) => {
      breakpoints.delete(label);
    },
    cancelAllBreakpoints: () => {
      breakpoints.clear();
    },
    parkedAt: () => parked,
    start: stepToStop,
    proceed: () => {
      parked = null;
      return stepToStop();
    },
  };
};

/** The whole story on the gcd machine: the book's four lines. */
export const breakpointSession = (): readonly string[] => {
  const session = makeBreakpointMachine();
  session.machine.writeRegister("a", 206);
  session.machine.writeRegister("b", 40);
  session.setBreakpoint("test-b", 2);
  const lines: string[] = [];
  lines.push(session.start());
  lines.push(session.proceed());
  for (;;) {
    const line = session.proceed();
    lines.push(line);
    if (line.startsWith("finished")) break;
  }
  session.cancelBreakpoint("test-b");
  const fresh = makeBreakpointMachine();
  fresh.machine.writeRegister("a", 206);
  fresh.machine.writeRegister("b", 40);
  lines.push(`cancel and restart: ${fresh.start().replace("finished: ", "")}`);
  return lines;
};

/** Exercise 5.19 answers. */
export const ex_5_19 = (): readonly string[] => breakpointSession();
