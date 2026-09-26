// SPDX-License-Identifier: GPL-3.0-only
import { runEvaluator } from "../../packages/ch5/src/04-eceval.js";
export const errorSignals = (): readonly string[] => {
  try {
    return runEvaluator("(/ 1 0)");
  } catch (error) {
    return [error instanceof Error ? error.message : String(error)];
  }
};
export const ex_5_30 = errorSignals;
