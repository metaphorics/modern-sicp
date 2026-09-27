// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  fibonacciRecursive,
  fibonacciReduced,
  runMachine,
} from "../../packages/ch5/src/01-register-machines.js";

/** Compares original and reduced Fibonacci traces and answers. */
export const ex_5_06 = (n: number) => ({
  original: runMachine(fibonacciRecursive, { n }),
  reduced: runMachine(fibonacciReduced, { n }),
});
