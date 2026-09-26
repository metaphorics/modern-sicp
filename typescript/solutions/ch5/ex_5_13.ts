// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  controllerRegisters,
  getRegisterContents,
  makeMachine,
  setRegisterContents,
} from "../../packages/ch5/src/02-simulator.js";
import { expectOk, gcdController } from "./ex_5_07.js";

/** The gcd machine with its registers derived from the controller, then
 * run: same answer as the hand-listed machine, and the derived register
 * set names exactly the controller's registers. */
export const derivedRegisterRuns = (): string[] => {
  const derived = controllerRegisters(gcdController).sort();
  const machine = expectOk(makeMachine(derived, arithmeticOperations, gcdController));
  expectOk(setRegisterContents(machine, "a", 206));
  expectOk(setRegisterContents(machine, "b", 40));
  expectOk(machine.start());
  const allocated = [...machine.registers.keys()].sort().join(" ");
  return [
    `derived registers: ${derived.join(" ")}`,
    `gcd(206, 40) = ${expectOk(getRegisterContents(machine, "a"))}`,
    `allocated registers: ${allocated}`,
  ];
};
