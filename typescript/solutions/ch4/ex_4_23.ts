// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.23: the text's analyze-sequence versus Alyssa's version. The
 * text's analyzer folds the execution procedures into one at analysis time,
 * so a one-expression body's execution procedure IS the body's procedure
 * and no sequence machinery runs at execution time. Alyssa's analyzer keeps
 * the list and walks it on every execution, so each run pays one walk. The
 * counters pin exactly that: leaf executions are identical, the walk
 * counter moves only for Alyssa's version.
 */
import { Effect, Ref } from "effect";
import type { ExecutionProcedure } from "../../packages/ch4/src/01-metacircular.js";
import {
  analyzeQuoted,
  analyzeSelfEvaluating,
  analyzeVariable,
  assignmentValue,
  assignmentVariable,
  beginActions,
  condToIf,
  defineVariableValue,
  definitionValue,
  definitionVariable,
  executeApplication,
  ifAlternative,
  ifConsequent,
  ifPredicate,
  isApplication,
  isAssignment,
  isBegin,
  isCond,
  isDefinition,
  isIf,
  isLambda,
  isQuoted,
  isSelfEvaluating,
  isTrue,
  isVariable,
  lambdaBody,
  lambdaParameters,
  makeProcedure,
  ok,
  operands,
  operator,
  setupEnvironment,
  setVariableValue,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Evaluate, ExecutionValue, Value } from "../../packages/ch4/src/core.js";
import {
  type EvaluationError,
  RuntimeError,
  UnknownSyntax,
} from "../../packages/ch4/src/errors.js";
import { cons, type List, nil, toArray } from "../../packages/ch4/src/list.js";
import { format, read } from "../../packages/ch4/src/read.js";

/** Runs of body procedures and of Alyssa's runtime walk, counted. */
export interface SequenceCounters {
  readonly leafRuns: Ref.Ref<number>;
  readonly walks: Ref.Ref<number>;
}

export const makeSequenceCounters = (): Effect.Effect<SequenceCounters> =>
  Effect.gen(function* () {
    const leafRuns = yield* Ref.make(0);
    const walks = yield* Ref.make(0);
    return { leafRuns, walks };
  });

/** A body procedure that counts one run per execution. */
const counted =
  (counters: SequenceCounters, proc: ExecutionProcedure): ExecutionProcedure =>
  (env) =>
    Effect.flatMap(
      Ref.update(counters.leafRuns, (n) => n + 1),
      () => proc(env),
    );

/** An analyzer over expressions, as both sequence versions consume it. */
export type ExpressionAnalyzer = (exp: Value) => ExecutionProcedure;

const emptySequenceFailure = (): Effect.Effect<Value, EvaluationError> =>
  Effect.fail(new RuntimeError({ message: "Empty sequence: ANALYZE", detail: "" }));

/**
 * The text's version: fold left at analysis time; a one-expression body
 * returns the body's own procedure and nothing wraps it at run time.
 */
export const analyzeSequenceText = (
  counters: SequenceCounters,
): ((exps: List<Value>, analyzeExp: ExpressionAnalyzer) => ExecutionProcedure) => {
  const sequentially =
    (proc1: ExecutionProcedure, proc2: ExecutionProcedure): ExecutionProcedure =>
    (env) =>
      Effect.flatMap(proc1(env), () => proc2(env));
  return (exps, analyzeExp) => {
    if (exps._tag === "Nil") {
      return () => emptySequenceFailure();
    }
    const loop = (first: ExecutionProcedure, rest: List<Value>): ExecutionProcedure =>
      rest._tag === "Nil"
        ? first
        : loop(sequentially(first, counted(counters, analyzeExp(rest.head))), rest.tail);
    return loop(counted(counters, analyzeExp(exps.head)), exps.tail);
  };
};

/**
 * Alyssa's version: analyze into a list, then walk the list on every
 * execution; each execution counts one walk.
 */
export const analyzeSequenceAlyssa =
  (
    counters: SequenceCounters,
  ): ((exps: List<Value>, analyzeExp: ExpressionAnalyzer) => ExecutionProcedure) =>
  (exps, analyzeExp) => {
    const procs: ReadonlyArray<ExecutionProcedure> = toArray(exps).map((exp) =>
      counted(counters, analyzeExp(exp)),
    );
    return (env) =>
      Effect.flatMap(
        Ref.update(counters.walks, (n) => n + 1),
        () => {
          const runFrom = (i: number): Effect.Effect<Value, EvaluationError> => {
            const proc = procs[i];
            if (proc === undefined) {
              return emptySequenceFailure();
            }
            return i === procs.length - 1
              ? proc(env)
              : Effect.flatMap(proc(env), () => runFrom(i + 1));
          };
          return runFrom(0);
        },
      );
  };

const listOf = (items: ReadonlyArray<Value>): List<Value> =>
  items.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);

const runOperandsLocal = (
  aprocs: ReadonlyArray<ExecutionProcedure>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> => {
  const args: Value[] = [];
  const runFrom = (i: number): Effect.Effect<List<Value>, EvaluationError> => {
    const aproc = aprocs[i];
    if (aproc === undefined) {
      return Effect.succeed(nil);
    }
    return Effect.flatMap(aproc(env), (value) => {
      args.push(value);
      return runFrom(i + 1);
    });
  };
  return Effect.map(runFrom(0), () => listOf(args));
};

/** A full analyzer whose begin and lambda bodies use the given sequence. */
export const makeSequenceAnalyzer = (
  analyzeSequence: (exps: List<Value>, analyzeExp: ExpressionAnalyzer) => ExecutionProcedure,
): ExpressionAnalyzer => {
  const analyzeExp: ExpressionAnalyzer = (exp) => {
    if (isSelfEvaluating(exp)) {
      return analyzeSelfEvaluating(exp);
    }
    if (isQuoted(exp)) {
      return analyzeQuoted(exp);
    }
    if (isVariable(exp)) {
      return analyzeVariable(exp);
    }
    if (isAssignment(exp)) {
      const variable = assignmentVariable(exp);
      const vproc = analyzeExp(assignmentValue(exp));
      return (env) =>
        Effect.flatMap(vproc(env), (value) =>
          Effect.map(setVariableValue(variable, value, env), () => ok),
        );
    }
    if (isDefinition(exp)) {
      const variable = definitionVariable(exp);
      const vproc = analyzeExp(definitionValue(exp));
      return (env) =>
        Effect.flatMap(vproc(env), (value) =>
          Effect.map(defineVariableValue(variable, value, env), () => ok),
        );
    }
    if (isIf(exp)) {
      const pproc = analyzeExp(ifPredicate(exp));
      const cproc = analyzeExp(ifConsequent(exp));
      const aproc = analyzeExp(ifAlternative(exp));
      return (env) =>
        Effect.flatMap(pproc(env), (predicate) => (isTrue(predicate) ? cproc(env) : aproc(env)));
    }
    if (isLambda(exp)) {
      const vars = lambdaParameters(exp);
      const bproc = analyzeSequence(lambdaBody(exp), analyzeExp);
      const bodyValue: ExecutionValue = { _tag: "Execution", run: bproc };
      return (env) => Effect.succeed(makeProcedure(vars, listOf([bodyValue]), env));
    }
    if (isBegin(exp)) {
      return analyzeSequence(beginActions(exp), analyzeExp);
    }
    if (isCond(exp)) {
      return analyzeExp(condToIf(exp));
    }
    if (isApplication(exp)) {
      const fproc = analyzeExp(operator(exp));
      const aprocs = toArray(operands(exp)).map(analyzeExp);
      return (env) =>
        Effect.flatMap(fproc(env), (procedure) =>
          Effect.flatMap(runOperandsLocal(aprocs, env), (args) =>
            executeApplication(procedure, args),
          ),
        );
    }
    return () => Effect.fail(new UnknownSyntax({ expr: format(exp) }));
  };
  return analyzeExp;
};

const runAnalyzedTimes = (
  analyze: Evaluate,
  source: string,
  times: number,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    Effect.flatMap(
      Effect.forEach(
        Array.from({ length: times }, () => read(source)),
        (exp) => analyze(exp, env),
      ),
      (values) => {
        const last = values[values.length - 1];
        return last !== undefined
          ? Effect.succeed(last)
          : Effect.die(new Error("no runs produced a value"));
      },
    ),
  );

/** Evaluates `source` through the text's analyzer `times` times. */
export const runWithTextSequence = (
  source: string,
  times: number,
): Effect.Effect<{ readonly value: Value; readonly counters: SequenceCounters }, EvaluationError> =>
  Effect.flatMap(makeSequenceCounters(), (counters) =>
    Effect.map(
      runAnalyzedTimes(
        (exp, env) => makeSequenceAnalyzer(analyzeSequenceText(counters))(exp)(env),
        source,
        times,
      ),
      (value) => ({ value, counters }),
    ),
  );

/** Evaluates `source` through Alyssa's analyzer `times` times. */
export const runWithAlyssaSequence = (
  source: string,
  times: number,
): Effect.Effect<{ readonly value: Value; readonly counters: SequenceCounters }, EvaluationError> =>
  Effect.flatMap(makeSequenceCounters(), (counters) =>
    Effect.map(
      runAnalyzedTimes(
        (exp, env) => makeSequenceAnalyzer(analyzeSequenceAlyssa(counters))(exp)(env),
        source,
        times,
      ),
      (value) => ({ value, counters }),
    ),
  );

export function ex_4_23(): string {
  return (
    "For a one-expression body, the text's execution procedure IS the body's own " +
    "execution procedure: the analysis-time fold returns it unchanged, so a run does no " +
    "sequence work at all. Alyssa's version still loops: every execution walks the " +
    "one-element proc list, calls the walk once, then runs the body. For a two-expression " +
    "body the text's fold has already built proc1-then-proc2 into the procedure tree, " +
    "while Alyssa's version walks the two-element list on every run. The counters show " +
    "identical body-procedure executions in both versions; only the walk counter " +
    "distinguishes them, and it moves once per execution for Alyssa's version and never " +
    "for the text's."
  );
}
