// SPDX-License-Identifier: GPL-3.0-only
import { runEvaluator } from "../../packages/ch5/src/04-eceval.js";
export const normalOrderBaseline = (): readonly string[] => runEvaluator("(+ 1 2)");
export const ex_5_25 = normalOrderBaseline;
