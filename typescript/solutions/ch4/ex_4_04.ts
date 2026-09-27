// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.4: and and or as new special forms. (and e1 ... en)
 * evaluates the expressions left to right: if any evaluates to false,
 * false is returned and the rest are never evaluated; otherwise the value
 * of the last expression is returned, and (and) with no expressions is
 * true. (or e1 ... en) returns the first value that is not false and
 * never evaluates the rest; (or) is false. The full dispatch below keeps
 * the recursion inside this evaluator, so and and or also work in nested
 * positions such as procedure bodies.
 */
import { Effect } from "effect";

import {
  applyPrimitiveProcedure,
  assignmentValue,
  assignmentVariable,
  beginActions,
  defineVariableValue,
  definitionValue,
  definitionVariable,
  extendEnvironment,
  falseValue,
  firstExp,
  firstOperand,
  ifAlternative,
  ifConsequent,
  ifPredicate,
  isAssignment,
  isBegin,
  isDefinition,
  isIf,
  isLambda,
  isLastExp,
  isPair,
  isQuoted,
  isSelfEvaluating,
  isTrue,
  isVariable,
  lambdaBody,
  lambdaParameters,
  lookupVariableValue,
  makeProcedure,
  noOperands,
  ok,
  operands,
  operator,
  restExps,
  restOperands,
  setVariableValue,
  taggedList,
  textOfQuotation,
  trueValue,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Evaluate, Value } from "../../packages/ch4/src/core.js";
import {
  type EvaluationError,
  NotAProcedure,
  RuntimeError,
  UnknownSyntax,
} from "../../packages/ch4/src/errors.js";
import type { Cons, List } from "../../packages/ch4/src/list.js";
import { cons, nil } from "../../packages/ch4/src/list.js";
import { format } from "../../packages/ch4/src/read.js";

/** The book's `and?`. */
export const isAnd = (exp: Value): exp is Cons<Value> => taggedList("and", exp);

/** The book's `or?`. */
export const isOr = (exp: Value): exp is Cons<Value> => taggedList("or", exp);

const tailOf = (exp: Cons<Value>): List<Value> => exp.tail;

export const evalWithAndOr: Evaluate = (exp, env) => {
  if (isSelfEvaluating(exp)) {
    return Effect.succeed(exp);
  }
  if (isVariable(exp)) {
    return lookupVariableValue(exp, env);
  }
  if (isQuoted(exp)) {
    return Effect.succeed(textOfQuotation(exp));
  }
  if (isAssignment(exp)) {
    return evalAssignmentLocal(exp, env);
  }
  if (isDefinition(exp)) {
    return evalDefinitionLocal(exp, env);
  }
  if (isAnd(exp)) {
    return evalConjunction(tailOf(exp), env);
  }
  if (isOr(exp)) {
    return evalDisjunction(tailOf(exp), env);
  }
  if (isIf(exp)) {
    return evalIfLocal(exp, env);
  }
  if (isLambda(exp)) {
    return Effect.succeed(makeProcedure(lambdaParameters(exp), lambdaBody(exp), env));
  }
  if (isBegin(exp)) {
    return evalSequenceLocal(beginActions(exp), env);
  }
  if (isPair(exp)) {
    return Effect.flatMap(evalWithAndOr(operator(exp), env), (procedure) =>
      Effect.flatMap(listOfValuesLocal(operands(exp), env), (args) => applyLocal(procedure, args)),
    );
  }
  return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
};

const evalConjunction = (
  conjuncts: List<Value>,
  env: Env,
): Effect.Effect<Value, EvaluationError> => {
  if (conjuncts._tag === "Nil") {
    return Effect.succeed(trueValue);
  }
  if (isLastExp(conjuncts)) {
    return evalWithAndOr(firstExp(conjuncts), env);
  }
  return Effect.flatMap(evalWithAndOr(firstExp(conjuncts), env), (value) =>
    isTrue(value) ? evalConjunction(restExps(conjuncts), env) : Effect.succeed(falseValue),
  );
};

const evalDisjunction = (
  disjuncts: List<Value>,
  env: Env,
): Effect.Effect<Value, EvaluationError> => {
  if (disjuncts._tag === "Nil") {
    return Effect.succeed(falseValue);
  }
  if (isLastExp(disjuncts)) {
    return evalWithAndOr(firstExp(disjuncts), env);
  }
  return Effect.flatMap(evalWithAndOr(firstExp(disjuncts), env), (value) =>
    isTrue(value) ? Effect.succeed(value) : evalDisjunction(restExps(disjuncts), env),
  );
};

const evalSequenceLocal = (seq: List<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
  if (seq._tag === "Nil") {
    return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
  }
  if (isLastExp(seq)) {
    return evalWithAndOr(firstExp(seq), env);
  }
  return Effect.flatMap(evalWithAndOr(firstExp(seq), env), () =>
    evalSequenceLocal(restExps(seq), env),
  );
};

const applyLocal = (procedure: Value, args: List<Value>): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag === "Primitive") {
    return applyPrimitiveProcedure(procedure, args);
  }
  if (procedure._tag === "Compound") {
    return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (newEnv) =>
      evalSequenceLocal(procedure.body, newEnv),
    );
  }
  return Effect.fail(new NotAProcedure({ value: format(procedure) }));
};

const listOfValuesLocal = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> =>
  noOperands(exps)
    ? Effect.succeed(nil)
    : Effect.flatMap(evalWithAndOr(firstOperand(exps), env), (first) =>
        Effect.map(listOfValuesLocal(restOperands(exps), env), (rest) => cons(first, rest)),
      );

const evalIfLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithAndOr(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate)
      ? evalWithAndOr(ifConsequent(exp), env)
      : evalWithAndOr(ifAlternative(exp), env),
  );

const evalAssignmentLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithAndOr(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithAndOr(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

export function ex_4_04(): string {
  return (
    "(and) with no conjuncts is true and (or) with no disjuncts is false; and evaluates its " +
    "expressions in turn and stops with false as soon as one is false, otherwise returning " +
    "the last value, while or stops with the first non-false value and otherwise returns " +
    "false. Because the dispatch re-enters itself for every subexpression, both forms also " +
    "work inside procedure bodies, where the module's own evaluate would never see them."
  );
}
