// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { factorialIterative, runMachine } from "../../packages/ch5/src/01-register-machines.js";

/** Runs the controller sequence for iterative factorial. */
export const ex_5_02 = (n: number) => runMachine(factorialIterative, { n });
