// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.25: unless breaks under applicative order. The same
 * unless-based factorial runs under two evaluators. The lazy
 * experiment delays the arms at the call site, the condition forces
 * only the chosen arm, and the recursion bottoms out at 120. The
 * strict core evaluator keeps the edition's strict-argument rule: every
 * operand evaluates before the procedure is entered, so the unused arm
 * runs — its fault reaches the caller before `unless` can choose — and
 * the strict factorial descends without ever reaching its base case.
 * The descent runs against a host budget on the recursive argument, so
 * the predicted non-termination answers as a typed budget error instead
 * of hanging the run.
 */
import { defineVariableValue, Session } from "../../packages/ch4/src/01-metacircular.js";
import { runLazySource } from "../../packages/ch4/src/02-lazy.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import { fail, ok } from "../../packages/ch4/src/runtime/errors.js";
import { makePrimitive, type Value } from "../../packages/ch4/src/runtime/value.js";
import { admitSource } from "../../packages/ch4/src/syntax/check.js";

/** The lazy factorial: `unless` forces only the arm it returns. */
export const lazyUnlessFactorialSource = `
const unless = (c: boolean, unchosen: number, chosen: number): number =>
  c ? force(chosen) : force(unchosen);
const fact = (n: number): number => unless(n === 1, delay(n * fact(n - 1)), delay(1));
fact(5);
`;

/** The strict side's shared setup: a counting arm and a failing arm. */
export const strictSetupSource = `
let marks = 0;
const mark = (v: number): number => {
  marks = marks + 1;
  return v;
};
const boom = (): number => 0;
const unlessStrict = (c: boolean, unchosen: number, chosen: number): number =>
  c ? chosen : unchosen;
const factStrict = (n: number): number => unlessStrict(n === 1, n * factStrict(descend(n - 1)), 1);
const descend = (n: number): number => n;
`;

/** A session with the strict setup installed; `boom` and `descend` are
 * replaced by the host's failing and budgeted primitives. */
export const strictEnv = (limit: number): { session: Session; env: Env; spent: () => number } => {
  const session = new Session("core");
  const env = session.globalEnv();
  const admission = admitSource(strictSetupSource);
  if (admission.ok) {
    session.execSequence(admission.program, env);
  }
  let steps = 0;
  defineVariableValue(
    "boom",
    makePrimitive("boom", (_args: ReadonlyArray<Value>) =>
      fail({
        tag: "bad-operand",
        operator: "boom",
        detail: "the unused arm ran before unless was entered",
      }),
    ),
    env,
  );
  defineVariableValue(
    "descend",
    makePrimitive("descend", (args: ReadonlyArray<Value>) => {
      steps += 1;
      return steps > limit
        ? fail({
            tag: "bad-operand",
            operator: "descend",
            detail: "the step budget ran out: factorial is still descending",
          })
        : ok(args[0]);
    }),
    env,
  );
  return { session, env, spent: () => steps };
};

/** The lazy factorial run through the named lazy experiment. */
export const runLazyFactorial = (): RunResult =>
  runLazySource(lazyUnlessFactorialSource, "lazy-memoized-experiment");

export function ex_4_25(): string {
  return (
    "Under the lazy experiment the arms are delayed and only the chosen one is forced, so " +
    "the unless-based factorial bottoms out at 120. Under strict arguments the unused arm " +
    "evaluates before unless is entered: the failing arm's fault reaches the caller first, " +
    "and the counter arm records its run even when the other arm is chosen. The strict " +
    'factorial descends forever and answers the typed budget error "the step budget ran ' +
    'out: factorial is still descending" instead of hanging.'
  );
}
