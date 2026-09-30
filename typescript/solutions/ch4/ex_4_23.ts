// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { ExecutionProcedure } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.23: analyze-sequence comparison. Both versions run over
 * one analyzer with counters attached to every body procedure: a
 * `leafRuns` counter counting execution-procedure runs and a `walks`
 * counter counting Alyssa's runtime walk. The text's version folds left
 * at analysis time, so a one-expression body returns the body's own
 * procedure and nothing sequence-shaped exists at run time. Alyssa's
 * version analyzes into a list and walks it on every execution,
 * incrementing the walk counter once per run. Each program is analyzed
 * once and executed five times, so run counts are comparable.
 */
import { analyze, Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, ok } from "../../packages/ch4/src/runtime/errors.js";
import type { Expr } from "../../packages/ch4/src/syntax/ast.js";

/** The two counters the comparison observes. */
export interface SequenceCounters {
  leafRuns: number;
  walks: number;
}

/** Fresh counters for one comparison run. */
export const makeCounters = (): SequenceCounters => ({ leafRuns: 0, walks: 0 });

const instrument =
  (procedure: ExecutionProcedure, counters: SequenceCounters): ExecutionProcedure =>
  (env: Env) => {
    counters.leafRuns += 1;
    return procedure(env);
  };

/** The text's version: fold left at analysis time. */
export const analyzeSequenceText = (
  items: ReadonlyArray<Expr>,
  counters: SequenceCounters,
): ExecutionProcedure => {
  if (items.length === 0) {
    return () => fail({ tag: "unknown-syntax", construct: "empty sequence" });
  }
  const procedures = items.map((item) => instrument(analyze(item), counters));
  return procedures.reduce((first, next) => (env: Env): Outcome => {
    const head = first(env);
    return head.tag === "error" ? head : next(env);
  });
};

/** Alyssa's version: analyze into a list and walk it on every execution. */
export const analyzeSequenceAlyssa = (
  items: ReadonlyArray<Expr>,
  counters: SequenceCounters,
): ExecutionProcedure => {
  if (items.length === 0) {
    return () => fail({ tag: "unknown-syntax", construct: "empty sequence" });
  }
  const procedures = items.map((item) => instrument(analyze(item), counters));
  return (env: Env): Outcome => {
    counters.walks += 1;
    let last: Outcome = ok(undefined);
    for (const procedure of procedures) {
      last = procedure(env);
      if (last.tag === "error") {
        return last;
      }
    }
    return last;
  };
};

/** Runs one analyzed sequence `runs` times in a fresh global frame. */
export const runSequence = (
  build: (counters: SequenceCounters) => ExecutionProcedure,
  runs: number,
): SequenceCounters => {
  const counters = makeCounters();
  const procedure = build(counters);
  for (let i = 0; i < runs; i += 1) {
    procedure(new Session("core").globalEnv());
  }
  return counters;
};

export function ex_4_23(): string {
  return (
    "The difference is the walk, not the body work. A one-expression body executed five " +
    "times: the text's version runs 5 leaf procedures and 0 walks — its execution " +
    "procedure IS the body's procedure — while Alyssa's runs 5 leaves and 5 walks. A " +
    "two-expression body: 10 leaves and 0 walks against 10 leaves and 5 walks. The text's " +
    "fold builds proc1-then-proc2 into the procedure tree at analysis time; Alyssa's " +
    "loops through the list on every run."
  );
}
