// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  type Machine,
  makeMachine,
  setRegisterContents,
} from "../../packages/ch5/src/02-simulator.js";
import { expectOk, gcdController } from "./ex_5_07.js";

/** The tracing machine: when the switch is on, every instruction's text
 * is appended to the transcript before the instruction executes. */
export const makeTracingMachine = (
  registerNames: string[],
): { machine: Machine; setTrace: (on: boolean) => void } => {
  let traceOn = false;
  const machine = expectOk(
    makeMachine(registerNames, arithmeticOperations, gcdController, {
      onInstruction: (m, inst) => {
        if (traceOn) m.transcript.push(inst.text);
      },
    }),
  );
  return { machine, setTrace: (on) => (traceOn = on) };
};

const runGcd = (machine: Machine): void => {
  expectOk(setRegisterContents(machine, "a", 206));
  expectOk(setRegisterContents(machine, "b", 40));
  expectOk(machine.start());
};

/** One traced gcd run: the transcript is exactly the executed
 * instructions, from the first (test (op =) (reg b) (const 0)) to the
 * final taken (branch (label gcd-done)), never the trailing label. */
export const tracedGcdTrace = (): string[] => {
  const { machine, setTrace } = makeTracingMachine(["a", "b", "t"]);
  setTrace(true);
  runGcd(machine);
  return [...machine.transcript];
};

/** The switch off leaves no trace lines. */
export const untracedRunTranscript = (): string[] => {
  const { machine, setTrace } = makeTracingMachine(["a", "b", "t"]);
  setTrace(false);
  runGcd(machine);
  return [...machine.transcript];
};
