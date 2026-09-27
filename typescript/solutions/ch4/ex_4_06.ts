// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.6: let expressions are derived expressions. The
 * transformation here is the book's: (let ((v1 e1) ... (vn en)) body)
 * becomes ((lambda (v1 ... vn) body) e1 ... en), built as data with the
 * module's make-lambda, and the eval clause simply evaluates the
 * combination. The full dispatch keeps the recursion inside this file, so
 * let works in nested positions such as procedure bodies.
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
  isSymbol,
  isTrue,
  isVariable,
  lambdaBody,
  lambdaParameters,
  lookupVariableValue,
  makeLambda,
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

const cadrOf = (exp: Cons<Value>): Value => {
  const rest = exp.tail;
  return rest._tag === "Cons" ? rest.head : nil;
};

const cddrOf = (exp: Cons<Value>): List<Value> => (exp.tail._tag === "Cons" ? exp.tail.tail : nil);

/** The book's `let?`: a let whose bindings slot holds a list, not a name. */
export const isLet = (exp: Value): exp is Cons<Value> =>
  taggedList("let", exp) && !isSymbol(cadrOf(exp));

const bindingsOf = (exp: Cons<Value>): List<Value> => {
  const bindings = cadrOf(exp);
  return bindings._tag === "Cons" || bindings._tag === "Nil" ? bindings : nil;
};

const bodyOf = (exp: Cons<Value>): List<Value> => cddrOf(exp);

const bindingVariable = (binding: Value): Value => (isPair(binding) ? binding.head : binding);

const bindingInitializer = (binding: Value): Value => (isPair(binding) ? cadrOf(binding) : binding);

const bindingParts = (bindings: List<Value>, part: (binding: Value) => Value): List<Value> =>
  bindings._tag === "Cons" ? cons(part(bindings.head), bindingParts(bindings.tail, part)) : nil;

/** The book's `let->combination`: the lambda form as data. */
export const letToCombination = (exp: Cons<Value>): Value =>
  cons(
    makeLambda(bindingParts(bindingsOf(exp), bindingVariable), bodyOf(exp)),
    bindingParts(bindingsOf(exp), bindingInitializer),
  );

export const evalWithLet: Evaluate = (exp, env) => {
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
  if (isLet(exp)) {
    return evalWithLet(letToCombination(exp), env);
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
    return Effect.flatMap(evalWithLet(operator(exp), env), (procedure) =>
      Effect.flatMap(listOfValuesLocal(operands(exp), env), (args) => applyLocal(procedure, args)),
    );
  }
  return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
};

const evalSequenceLocal = (seq: List<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
  if (seq._tag === "Nil") {
    return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
  }
  if (isLastExp(seq)) {
    return evalWithLet(firstExp(seq), env);
  }
  return Effect.flatMap(evalWithLet(firstExp(seq), env), () =>
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
    : Effect.flatMap(evalWithLet(firstOperand(exps), env), (first) =>
        Effect.map(listOfValuesLocal(restOperands(exps), env), (rest) => cons(first, rest)),
      );

const evalIfLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithLet(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate) ? evalWithLet(ifConsequent(exp), env) : evalWithLet(ifAlternative(exp), env),
  );

const evalAssignmentLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithLet(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithLet(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

export function ex_4_06(): string {
  return (
    "let is a derived expression: let->combination rebuilds it as the combination " +
    "((lambda (v1 ... vn) body) e1 ... en) using the module's make-lambda, and the eval " +
    "clause is just evaluate(letToCombination(exp), env). The transformed data is " +
    "structurally equal to the hand-written lambda form, and evaluating the let evaluates " +
    "to the same value as evaluating that combination."
  );
}
