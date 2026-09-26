// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  assign,
  type ControllerLine,
  c,
  jump,
  makeMachine,
  mark,
  renderMachineError,
} from "../../packages/ch5/src/02-simulator.js";
import { expectError } from "./ex_5_07.js";

/** The book's snippet: two definitions of here, and there defined only
 * after the second here. */
const doublyDefinedLabelController: ControllerLine[] = [
  mark("start"),
  jump("here"),
  mark("here"),
  assign("a", c(0)),
  jump("there"),
  mark("here"),
  assign("a", c(1)),
  jump("here"),
  mark("there"),
];

/** Assembling the snippet: the typed duplicate-label error, never a
 * machine. */
export const duplicateLabelOutcome = (): string => {
  const outcome = makeMachine(["a"], arithmeticOperations, doublyDefinedLabelController);
  return outcome.ok ? "assembled" : renderMachineError(expectError(outcome));
};
