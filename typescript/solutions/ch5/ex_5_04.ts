// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  exponentIterative,
  exponentRecursive,
  runMachine,
} from "../../packages/ch5/src/01-register-machines.js";

/** Runs both exponentiation controllers. */
export const ex_5_04 = (b: number, n: number) => ({
  recursive: runMachine(exponentRecursive, { b, n }),
  iterative: runMachine(exponentIterative, { b, n }),
});
