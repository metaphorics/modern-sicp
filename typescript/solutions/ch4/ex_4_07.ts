// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.7: let* binds sequentially, each initializer seeing the
 * previous bindings. The transformation is the book's rewrite:
 * (let* ((v1 e1) (v2 e2) ...) body) becomes
 * (let ((v1 e1)) (let* ((v2 e2) ...) body)), and an empty bindings list
 * leaves only the body. Because the expansion produces plain let forms,
 * the dispatch here installs let as well, which answers the book's
 * question: the eval clause (eval (let*->nested-lets exp) env) is enough
 * once let is a clause of the same evaluator.
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
  sequenceToExp,
  setVariableValue,
  symbol,
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

/** The book's `let?` for plain let. */
export const isLet = (exp: Value): exp is Cons<Value> =>
  taggedList("let", exp) && !isSymbol(cadrOf(exp));

/** The book's `let*?`. */
export const isLetStar = (exp: Value): exp is Cons<Value> => taggedList("let*", exp);

const bindingsOf = (exp: Cons<Value>): List<Value> => {
  const bindings = cadrOf(exp);
  return bindings._tag === "Cons" || bindings._tag === "Nil" ? bindings : nil;
};

const bodyOf = (exp: Cons<Value>): List<Value> => cddrOf(exp);

const makeLet = (bindings: List<Value>, body: List<Value>): Value =>
  cons(symbol("let"), cons(bindings, body));

const makeLetStar = (bindings: List<Value>, body: List<Value>): Value =>
  cons(symbol("let*"), cons(bindings, body));

/** The book's `let*->nested-lets`: the outermost binding becomes a let
 * around the rest, until no bindings are left. */
export const letStarToNestedLets = (exp: Value): Value => {
  if (!isPair(exp)) {
    return exp;
  }
  const bindings = bindingsOf(exp);
  const body = bodyOf(exp);
  if (bindings._tag === "Nil") {
    return sequenceToExp(body);
  }
  const rest = bindings.tail;
  const inner: List<Value> =
    rest._tag === "Nil" ? body : cons(letStarToNestedLets(makeLetStar(rest, body)), nil);
  return makeLet(cons<Value>(bindings.head, nil), inner);
};

const bindingVariable = (binding: Value): Value => (isPair(binding) ? binding.head : binding);

const bindingInitializer = (binding: Value): Value => (isPair(binding) ? cadrOf(binding) : binding);

const bindingParts = (bindings: List<Value>, part: (binding: Value) => Value): List<Value> =>
  bindings._tag === "Cons" ? cons(part(bindings.head), bindingParts(bindings.tail, part)) : nil;

/** The plain-let combination of exercise 4.6, needed to evaluate the
 * nested let forms this expansion produces. */
export const letToCombination = (exp: Cons<Value>): Value =>
  cons(
    makeLambda(bindingParts(bindingsOf(exp), bindingVariable), bodyOf(exp)),
    bindingParts(bindingsOf(exp), bindingInitializer),
  );

export const evalWithLetStar: Evaluate = (exp, env) => {
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
  if (isLetStar(exp)) {
    return evalWithLetStar(letStarToNestedLets(exp), env);
  }
  if (isLet(exp)) {
    return evalWithLetStar(letToCombination(exp), env);
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
    return Effect.flatMap(evalWithLetStar(operator(exp), env), (procedure) =>
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
    return evalWithLetStar(firstExp(seq), env);
  }
  return Effect.flatMap(evalWithLetStar(firstExp(seq), env), () =>
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
    : Effect.flatMap(evalWithLetStar(firstOperand(exps), env), (first) =>
        Effect.map(listOfValuesLocal(restOperands(exps), env), (rest) => cons(first, rest)),
      );

const evalIfLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithLetStar(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate)
      ? evalWithLetStar(ifConsequent(exp), env)
      : evalWithLetStar(ifAlternative(exp), env),
  );

const evalAssignmentLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithLetStar(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithLetStar(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

export function ex_4_07(): string {
  return (
    "let*->nested-lets peels off the outermost binding as a one-binding let around the rest " +
    "of the bindings, so each initializer is evaluated in the environment of the previous " +
    "ones; with no bindings left only the body remains. Since the expansion is built only " +
    "of let forms, adding the eval clause (eval (let*->nested-lets exp) env) to an " +
    "evaluator that already has a let clause is sufficient; no non-derived expressions are " +
    "needed."
  );
}
