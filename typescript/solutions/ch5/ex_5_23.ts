// SPDX-License-Identifier: GPL-3.0-only
import { runEvaluator } from "../../packages/ch5/src/04-eceval.js";
export const derivedExpressions = (): readonly string[] => runEvaluator("(if (= 1 2) 3 4) (* 2 3)");
export const ex_5_23 = derivedExpressions;
