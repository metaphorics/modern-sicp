// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ex_5_46 as fibRatios } from "./ex_5_45.ts";

/** Exercise 5.46 delegates to the shared measurement with the
 * tree-recursive Fibonacci, whose ratios show the compiler's
 * effectiveness on tree recursion. */
export const ex_5_46 = (): readonly string[] => fibRatios();
