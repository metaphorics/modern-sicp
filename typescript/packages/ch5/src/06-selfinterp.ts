// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5

/**
 * The self-interpretation driver (host-subsets grammar section 5): runs the
 * guest evaluator witness — itself valid admitted host-subset source — under
 * each teaching engine so the same program demonstrates that a guest
 * evaluator written in the subset executes on the direct evaluator, the
 * explicit-control machine, and the compiled machine alike.
 */
import { type RunResult, runSource } from "@sicp-ts/ch4/01-metacircular";
import { runEvaluator } from "./04-eceval.ts";
import { compileAndRun } from "./05-compilation.ts";

/** The engines a self-interpretation run can target. */
export type SelfInterpretationEngine = "direct" | "eceval" | "compiled";

/** Runs the guest-source evaluator witness on one engine. */
export const runSelfInterpretation = (
  engine: SelfInterpretationEngine,
  source: string,
): RunResult => {
  if (engine === "direct") {
    return runSource(source);
  }
  if (engine === "eceval") {
    return runEvaluator(source);
  }
  return compileAndRun(source);
};
