// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.22: let in the analyzed evaluator. A let is analyzed into the
 * execution procedure of its lambda combination, exactly as the direct
 * evaluator derives let to a combination. The analyzer here is complete
 * (every clause routes through the local analyzer), because the module's
 * analyze helpers recurse through the module's analyze and would leave a
 * let nested in an if, lambda, or application body unanalyzed.
 */
import { Effect } from "effect";
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
  symbol,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Evaluate, ExecutionValue, Value } from "../../packages/ch4/src/core.js";
import {
  type EvaluationError,
  RuntimeError,
  UnknownSyntax,
} from "../../packages/ch4/src/errors.js";
import { type Cons, cons, type List, nil, toArray } from "../../packages/ch4/src/list.js";
import { format, read } from "../../packages/ch4/src/read.js";

export const isLet = (exp: Value): exp is Cons<Value> =>
  exp._tag === "Cons" && exp.head._tag === "Symbol" && exp.head.name === "let";

interface Bindings {
  readonly names: Value[];
  readonly inits: Value[];
  readonly body: List<Value>;
}

const readBindings = (exp: Cons<Value>): Bindings => {
  const bindingList = exp.tail;
  if (bindingList._tag !== "Cons") {
    throw new RuntimeError({ message: "malformed let: missing bindings", detail: format(exp) });
  }
  const names: Value[] = [];
  const inits: Value[] = [];
  const first = bindingList.head;
  let rest: List<Value>;
  if (first._tag === "Cons" || first._tag === "Nil") {
    rest = first;
  } else {
    throw new RuntimeError({
      message: "malformed let: bindings must be a list",
      detail: format(first),
    });
  }
  while (rest._tag === "Cons") {
    const binding = rest.head;
    if (binding._tag !== "Cons" || binding.tail._tag !== "Cons") {
      throw new RuntimeError({ message: "malformed let binding", detail: format(binding) });
    }
    names.push(binding.head);
    inits.push(binding.tail.head);
    rest = rest.tail;
  }
  return { names, inits, body: bindingList.tail };
};

/** The book's let->combination: ((lambda (names...) body...) inits...). */
export const letToCombination = (exp: Cons<Value>): Value => {
  const { names, inits, body } = readBindings(exp);
  const lambda = cons<Value>(symbol("lambda"), cons(listOf(names), body));
  return cons<Value>(lambda, listOf(inits));
};

const listOf = (items: ReadonlyArray<Value>): List<Value> =>
  items.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);

const analyzeSequenceLocal = (
  analyzeExp: (exp: Value) => ExecutionProcedure,
  exps: List<Value>,
): ExecutionProcedure => {
  if (exps._tag === "Nil") {
    return () => Effect.fail(new RuntimeError({ message: "Empty sequence: ANALYZE", detail: "" }));
  }
  const loop = (first: ExecutionProcedure, rest: List<Value>): ExecutionProcedure =>
    rest._tag === "Nil"
      ? first
      : loop((env) => Effect.flatMap(first(env), () => analyzeExp(rest.head)(env)), rest.tail);
  return loop(analyzeExp(exps.head), exps.tail);
};

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

const makeAnalyzer = (): ((exp: Value) => ExecutionProcedure) => {
  const analyzeExp = (exp: Value): ExecutionProcedure => {
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
    if (isLet(exp)) {
      return analyzeExp(letToCombination(exp));
    }
    if (isLambda(exp)) {
      const vars = lambdaParameters(exp);
      const bproc = analyzeSequenceLocal(analyzeExp, lambdaBody(exp));
      const bodyValue: ExecutionValue = { _tag: "Execution", run: bproc };
      return (env) => Effect.succeed(makeProcedure(vars, listOf([bodyValue]), env));
    }
    if (isBegin(exp)) {
      return analyzeSequenceLocal(analyzeExp, beginActions(exp));
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

const analyzer = makeAnalyzer();

/** The exercise's clause: a let analyzes into its combination's procedure. */
export const analyzeLet = (exp: Cons<Value>): ExecutionProcedure => analyzer(letToCombination(exp));

/** The analyzed evaluator with let support. */
export const evalAnalyzedWithLet: Evaluate = (exp, env) => analyzer(exp)(env);

/** Evaluates forms in order in ONE global environment, analyzed, with let. */
export const runAnalyzed = (
  sources: ReadonlyArray<string>,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    Effect.flatMap(
      Effect.forEach(sources, (source) => evalAnalyzedWithLet(read(source), env)),
      (values) => {
        const last = values[values.length - 1];
        return last !== undefined
          ? Effect.succeed(last)
          : Effect.die(new Error("no forms evaluated"));
      },
    ),
  );

export function ex_4_22(): string {
  return (
    "A let is a derived expression, so analyzing it means analyzing the combination it " +
    "stands for: analyze-let analyzes ((lambda (names...) body...) inits...) and hands " +
    "back that application's execution procedure. Nothing else changes; a let nested in " +
    "a lambda body analyzes once with the closure, and each call runs the combination " +
    "with no let machinery left at run time."
  );
}
