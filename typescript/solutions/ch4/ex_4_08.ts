// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.8: named let. (let name ((v1 e1) ...) body) binds name to a
 * procedure of the binding variables over the body and then calls it with
 * the initializers, so the body can recur through name. Following the
 * book's hint, the transformation wraps a helper lambda whose parameter is
 * the loop name itself: ((lambda (name) (set! name (lambda (v1 ...) ...)
 * body) (name e1 ...)) 'ok). The parameter is the name the body refers
 * to, set! installs the procedure into that binding, and the final call
 * starts the loop; the throwaway initial actual is the fresh symbol ok,
 * standing in for the book's *unassigned*.
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
import { cons, list, nil } from "../../packages/ch4/src/list.js";
import { format } from "../../packages/ch4/src/read.js";

const cadrOf = (exp: Cons<Value>): Value => {
  const rest = exp.tail;
  return rest._tag === "Cons" ? rest.head : nil;
};

const caddrOf = (exp: Cons<Value>): Value => {
  const afterTail = exp.tail;
  return afterTail._tag === "Cons" && afterTail.tail._tag === "Cons" ? afterTail.tail.head : nil;
};

const cddrOf = (exp: Cons<Value>): List<Value> => (exp.tail._tag === "Cons" ? exp.tail.tail : nil);

/** The book's plain `let?`: bindings slot is a list, not a name. */
export const isLet = (exp: Value): exp is Cons<Value> =>
  taggedList("let", exp) && !isSymbol(cadrOf(exp));

/** A named let: the bindings slot is the loop's name. */
export const isNamedLet = (exp: Value): exp is Cons<Value> =>
  taggedList("let", exp) && isSymbol(cadrOf(exp));

const letBindingsOf = (exp: Cons<Value>): List<Value> => {
  const bindings = cadrOf(exp);
  return bindings._tag === "Cons" || bindings._tag === "Nil" ? bindings : nil;
};

const letBodyOf = (exp: Cons<Value>): List<Value> => cddrOf(exp);

const namedBindingsOf = (exp: Cons<Value>): List<Value> => {
  const bindings = caddrOf(exp);
  return bindings._tag === "Cons" || bindings._tag === "Nil" ? bindings : nil;
};

const namedBodyOf = (exp: Cons<Value>): List<Value> => {
  const afterBindings = cddrOf(exp);
  return afterBindings._tag === "Cons" ? afterBindings.tail : nil;
};

const bindingVariable = (binding: Value): Value => (isPair(binding) ? binding.head : binding);

const bindingInitializer = (binding: Value): Value => (isPair(binding) ? cadrOf(binding) : binding);

const bindingParts = (bindings: List<Value>, part: (binding: Value) => Value): List<Value> =>
  bindings._tag === "Cons" ? cons(part(bindings.head), bindingParts(bindings.tail, part)) : nil;

/** The book's named-let transformation: bind the loop name with set! and
 * call the loop once with the initializers. */
export const namedLetToCombination = (exp: Cons<Value>): Value => {
  const loopName = cadrOf(exp);
  if (!isSymbol(loopName)) {
    throw new Error("named let: the loop name must be a symbol");
  }
  const bindings = namedBindingsOf(exp);
  const formals = bindingParts(bindings, bindingVariable);
  const initializers = bindingParts(bindings, bindingInitializer);
  const initialActual = cons(symbol("quote"), cons(symbol("ok"), nil));
  const update = cons(
    symbol("set!"),
    cons(loopName, cons(makeLambda(formals, namedBodyOf(exp)), nil)),
  );
  const startCall = cons(loopName, initializers);
  const combiner = makeLambda(list(loopName), cons(update, cons(startCall, nil)));
  return cons(combiner, cons(initialActual, nil));
};

/** The plain-let combination of exercise 4.6, so both let shapes share
 * the same evaluator. */
export const letToCombination = (exp: Cons<Value>): Value =>
  cons(
    makeLambda(bindingParts(letBindingsOf(exp), bindingVariable), letBodyOf(exp)),
    bindingParts(letBindingsOf(exp), bindingInitializer),
  );

export const evalWithNamedLet: Evaluate = (exp, env) => {
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
  if (isNamedLet(exp)) {
    return evalWithNamedLet(namedLetToCombination(exp), env);
  }
  if (isLet(exp)) {
    return evalWithNamedLet(letToCombination(exp), env);
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
    return Effect.flatMap(evalWithNamedLet(operator(exp), env), (procedure) =>
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
    return evalWithNamedLet(firstExp(seq), env);
  }
  return Effect.flatMap(evalWithNamedLet(firstExp(seq), env), () =>
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
    : Effect.flatMap(evalWithNamedLet(firstOperand(exps), env), (first) =>
        Effect.map(listOfValuesLocal(restOperands(exps), env), (rest) => cons(first, rest)),
      );

const evalIfLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithNamedLet(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate)
      ? evalWithNamedLet(ifConsequent(exp), env)
      : evalWithNamedLet(ifAlternative(exp), env),
  );

const evalAssignmentLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithNamedLet(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithNamedLet(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

export function ex_4_08(): string {
  return (
    "A named let becomes the combination ((lambda (name) (set! name (lambda formals body)) " +
    "(name inits)) 'ok): the helper lambda binds the loop name, set! replaces that binding " +
    "with the loop procedure, and the final call starts the loop with the initializers, so " +
    "the body can recur through the name. Named let of Fibonacci 10 evaluates to 55, and " +
    "plain let still evaluates in the same evaluator."
  );
}
