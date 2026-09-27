// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  assemble,
  type Machine,
  makeNewMachine,
  renderAssemblySummary,
} from "../../packages/ch5/src/02-simulator.js";
import { gcdController } from "./ex_5_07.js";

/** Assembles the gcd controller once more and takes the summary the
 * assembler gathered. */
const gcdSummary = (machine: Machine) => {
  const program = assemble(gcdController, machine);
  if (!program.ok) throw new Error("assembly failed");
  return program.value.summary;
};

/** The 5.12 summary of the gcd machine: the instruction census, the
 * registers used, the labels. */
export const gcdMachineSummary = (): string => {
  const machine = makeNewMachine(["a", "b", "t"], arithmeticOperations);
  return renderAssemblySummary(gcdSummary(machine));
};
