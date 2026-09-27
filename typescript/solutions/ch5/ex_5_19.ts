// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  getRegisterContents,
  type Machine,
  makeMachine,
  setRegisterContents,
} from "../../packages/ch5/src/02-simulator.js";
import { expectOk, gcdController } from "./ex_5_07.js";

/** One breakpoint: stop before the instruction at the address executes
 * for the every-th time. `stopped` records a consumed stop so the
 * resumed arrival runs and counts exactly once. */
interface Breakpoint {
  readonly address: number;
  readonly every: number;
  executions: number;
  stopped: boolean;
}

/** The breakpoint machine: the park gate checks the breakpoints before
 * every instruction and parks the machine before an instruction's n-th
 * execution, and again before each later n-th; proceed resumes, and a
 * fresh start re-arms the schedule. */
export const makeBreakpointMachine = (
  registerNames: string[],
): {
  machine: Machine;
  setBreakpoint: (label: string, n: number) => void;
  cancelBreakpoint: (label: string) => void;
  cancelAllBreakpoints: () => void;
} => {
  const breakpoints = new Map<string, Breakpoint>();
  const gate = (m: Machine): string | null => {
    for (const [label, breakpoint] of breakpoints) {
      if (breakpoint.address !== m.pc) continue;
      if (
        !breakpoint.stopped &&
        breakpoint.executions % breakpoint.every === breakpoint.every - 1
      ) {
        breakpoint.stopped = true;
        return label;
      }
      breakpoint.stopped = false;
      breakpoint.executions += 1;
      return null;
    }
    return null;
  };
  const machine = expectOk(
    makeMachine(registerNames, arithmeticOperations, gcdController, {
      gate,
      onRestart: () => {
        for (const breakpoint of breakpoints.values()) breakpoint.stopped = false;
      },
    }),
  );
  return {
    machine,
    setBreakpoint: (label, n) => {
      const address = machine.labels.get(label);
      if (address === undefined) throw new Error(`no such label: ${label}`);
      breakpoints.set(label, { address, every: n, executions: 0, stopped: false });
    },
    cancelBreakpoint: (label) => {
      breakpoints.delete(label);
    },
    cancelAllBreakpoints: () => {
      breakpoints.clear();
    },
  };
};

/** One breakpoint session on the gcd machine: stop before the second
 * and fourth execution of test-b, reading the registers at each stop,
 * proceed to the answer, then cancel and restart straight through. */
export const breakpointSession = (): string[] => {
  const { machine, setBreakpoint, cancelBreakpoint } = makeBreakpointMachine(["a", "b", "t"]);
  expectOk(setRegisterContents(machine, "a", 206));
  expectOk(setRegisterContents(machine, "b", 40));
  setBreakpoint("test-b", 2);
  expectOk(machine.start());
  const lines: string[] = [];
  while (machine.parkedAt !== null) {
    lines.push(
      `break at ${machine.parkedAt}: a = ${expectOk(getRegisterContents(machine, "a"))}, b = ${expectOk(getRegisterContents(machine, "b"))}`,
    );
    expectOk(machine.proceed());
  }
  lines.push(`finished: gcd(206, 40) = ${expectOk(getRegisterContents(machine, "a"))}`);
  cancelBreakpoint("test-b");
  expectOk(machine.start());
  lines.push(`cancel and restart: gcd(206, 40) = ${expectOk(getRegisterContents(machine, "a"))}`);
  return lines;
};
