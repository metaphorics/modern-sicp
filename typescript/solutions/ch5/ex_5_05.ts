// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  factorialRecursive,
  fibonacciRecursive,
  runMachine,
} from "../../packages/ch5/src/01-register-machines.js";

/** Produces factorial and Fibonacci hand-simulation traces. */
export const ex_5_05 = (factorialN: number, fibonacciN: number) => ({
  factorial: runMachine(factorialRecursive, { n: factorialN }),
  fibonacci: runMachine(fibonacciRecursive, { n: fibonacciN }),
});
