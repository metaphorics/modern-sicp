// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.2

/**
 * The named lazy experiments (host-subsets grammar section 6): two separate
 * execution modes over the same checked syntax, never core TypeScript
 * features. `lazy-memoized-experiment` forces each delay once and stores the
 * value; `lazy-recompute-experiment` recomputes on every force. Both are
 * admitted only as the explicitly marked `delay`/`force` AST extension — not
 * as parser aliases for ordinary calls — and both leave core strictness
 * untouched. Force counts (the 4.2 counting exercises) come back as data.
 */
import { executeProgram, type RunResult, Session } from "./01-metacircular.ts";
import { format } from "./read.ts";
import type { Env } from "./runtime/env.ts";
import { fail, type Outcome } from "./runtime/errors.ts";
import { admitSource } from "./syntax/check.ts";

/** The two lazy execution modes. */
export type LazyMode = "lazy-memoized-experiment" | "lazy-recompute-experiment";

/** One lazy run with its thunk-evaluation count. */
export interface LazyRun {
  readonly result: RunResult;
  readonly evaluations: number;
}

/** Reads, admits, and runs one source unit in a named lazy mode. */
export const runLazySource = (text: string, mode: LazyMode): RunResult => {
  const admission = admitSource(text, mode);
  if (!admission.ok) {
    return {
      outcome: fail({
        tag: "unknown-syntax",
        construct:
          admission.diagnostics[0]?.construct ?? `TS${admission.hostDiagnostics[0]?.code ?? 0}`,
      }),
      transcript: [],
    };
  }
  const session = new Session(mode);
  return { outcome: executeProgram(admission.program, session), transcript: session.transcript };
};

/** Runs a lazy unit and reports how many thunks were forced. */
export const runLazyCounted = (text: string, mode: LazyMode): LazyRun => {
  const admission = admitSource(text, mode);
  if (!admission.ok) {
    return {
      result: {
        outcome: fail({
          tag: "unknown-syntax",
          construct:
            admission.diagnostics[0]?.construct ?? `TS${admission.hostDiagnostics[0]?.code ?? 0}`,
        }),
        transcript: [],
      },
      evaluations: 0,
    };
  }
  const session = new Session(mode);
  const outcome = executeProgram(admission.program, session);
  return { result: { outcome, transcript: session.transcript }, evaluations: session.evaluations };
};

/** The lazy driver loop: session inputs in one lazy environment. */
export const lazyDriverLoop = (
  env: Env,
  inputs: ReadonlyArray<string>,
  mode: LazyMode,
): RunResult => {
  const session = new Session(mode);
  const lines: string[] = [];
  let last: Outcome = fail({ tag: "unknown-syntax", construct: "empty-session" });
  for (const input of inputs) {
    const admission = admitSource(input, mode);
    if (!admission.ok) {
      return {
        outcome: fail({
          tag: "unknown-syntax",
          construct:
            admission.diagnostics[0]?.construct ?? `TS${admission.hostDiagnostics[0]?.code ?? 0}`,
        }),
        transcript: lines,
      };
    }
    lines.push(";;; Lazy-Eval input:", input);
    const completion = session.execSequence(admission.program, env);
    last =
      completion.tag === "normal" || completion.tag === "return"
        ? { tag: "ok", value: completion.value }
        : completion.tag === "throw"
          ? fail({ tag: "guest-throw", value: completion.value })
          : completion.tag === "error"
            ? fail(completion.error)
            : fail({ tag: "unknown-syntax", construct: `unexpected-${completion.tag}` });
    if (last.tag === "error") {
      lines.push(";;; Lazy-Eval error:", JSON.stringify(last.error));
      return { outcome: last, transcript: lines };
    }
    lines.push(";;; Lazy-Eval value:", format(last.value));
  }
  return { outcome: last, transcript: lines };
};
