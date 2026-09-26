// SPDX-License-Identifier: GPL-3.0-only
import { runEvaluator } from "../../packages/ch5/src/04-eceval.js";
export const basicConditionals = (): readonly string[] =>
  runEvaluator("(if (= 1 1) 7 8) (if (= 1 2) 7 8)");
export const ex_5_24 = basicConditionals;
