// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  type Machine,
  makeMachine,
  setRegisterContents,
} from "../../packages/ch5/src/02-simulator.js";
import { expectOk, gcdController } from "./ex_5_07.js";

/** The label-aware tracing machine: the label currently in effect, the
 * most recently passed label definition, is printed ahead of each traced
 * instruction, so the trace reads like the controller listing. */
export const makeLabelTracingMachine = (
  registerNames: string[],
): { machine: Machine; setTrace: (on: boolean) => void } => {
  let traceOn = false;
  const machine = expectOk(
    makeMachine(registerNames, arithmeticOperations, gcdController, {
      onInstruction: (m, inst) => {
        if (!traceOn) return;
        const label = m.labelInEffect(m.pc);
        m.transcript.push(label === null ? inst.text : `${label}: ${inst.text}`);
      },
    }),
  );
  return { machine, setTrace: (on) => (traceOn = on) };
};

/** The traced gcd run with labels: every executed line is named by the
 * label in effect, test-b for the loop, gcd-done for nothing (the
 * machine halts on reaching it). */
export const labelTracedGcdTrace = (): string[] => {
  const { machine, setTrace } = makeLabelTracingMachine(["a", "b", "t"]);
  setTrace(true);
  expectOk(setRegisterContents(machine, "a", 206));
  expectOk(setRegisterContents(machine, "b", 40));
  expectOk(machine.start());
  return [...machine.transcript];
};
