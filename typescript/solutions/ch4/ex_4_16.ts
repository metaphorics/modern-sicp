// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.16: internal defines, scanned out. Three parts: a lookup
 * that fails when it finds the *unassigned* marker; scan-out-defines,
 * which rewrites a body's defines into one let of '*unassigned* bindings
 * followed by set!s; and the scan installed in make-procedure, so each
 * closure's body is transformed once, at creation, never at every call.
 * The dispatch below is complete and adds let as a derived expression,
 * which the scanned bodies need.
 */
import { Effect } from "effect";

import {
  applyPrimitiveProcedure,
  assignmentValue,
  assignmentVariable,
  beginActions,
  condToIf,
  defineVariableValue,
  definitionValue,
  definitionVariable,
  extendEnvironment,
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
  lookupVariableValue,
  makeLambda,
  makeProcedure,
  ok,
  operands,
  operator,
  setVariableValue,
  symbol,
  taggedList,
  textOfQuotation,
} from "../../packages/ch4/src/01-metacircular.js";
import type {
  CompoundProc,
  Env,
  Evaluate,
  SymbolValue,
  Value,
} from "../../packages/ch4/src/core.js";
import {
  type EvaluationError,
  NotAProcedure,
  RuntimeError,
  UnknownSyntax,
} from "../../packages/ch4/src/errors.js";
import { type Cons, cons, type List, list, nil, toArray } from "../../packages/ch4/src/list.js";
import { format, ReadError, read } from "../../packages/ch4/src/read.js";

const UNASSIGNED = "*unassigned*";

export const unassignedMarker: SymbolValue = symbol(UNASSIGNED);

export const isUnassigned = (value: Value): boolean =>
  value._tag === "Symbol" && value.name === UNASSIGNED;

// (a) lookup that refuses the marker -------------------------------

export const lookupVariableValueScanned = (
  variable: SymbolValue,
  env: Env,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(lookupVariableValue(variable, env), (value) =>
    isUnassigned(value)
      ? Effect.fail(
          new RuntimeError({
            message: "lookup: read a name before its set!",
            detail: variable.name,
          }),
        )
      : Effect.succeed(value),
  );

// (b) scan-out-defines -----------------------------------------------

const quoted = (value: Value): Value => list(symbol("quote"), value);

/** Rewrites a body so no define remains: each internal define becomes a
 * let binding initialized to '*unassigned* plus a set! at the front, and
 * the remaining expressions follow. A body with no defines is unchanged. */
export const scanOutDefines = (body: List<Value>): List<Value> => {
  const names: SymbolValue[] = [];
  const assignments: Value[] = [];
  const rest: Value[] = [];
  let cursor = body;
  while (cursor._tag === "Cons") {
    const exp = cursor.head;
    if (isDefinition(exp)) {
      const name = definitionVariable(exp);
      names.push(name);
      assignments.push(list(symbol("set!"), name, definitionValue(exp)));
    } else {
      rest.push(exp);
    }
    cursor = cursor.tail;
  }
  if (names.length === 0) {
    return body;
  }
  const bindings = names.map((name) => list(name, quoted(unassignedMarker)));
  const letBody = [...assignments, ...rest];
  return list(cons(symbol("let"), cons(fromValues(bindings), fromValues(letBody))));
};

// let as a derived expression, the shape the scan emits ---------------

export const isLet = (exp: Value): exp is Cons<Value> => taggedList("let", exp);

interface LetBinding {
  readonly name: SymbolValue;
  readonly value: Value;
}

const parseBinding = (binding: Value): LetBinding | undefined => {
  if (binding._tag !== "Cons" || binding.head._tag !== "Symbol") {
    return undefined;
  }
  const value = binding.tail._tag === "Cons" ? binding.tail.head : nil;
  return { name: binding.head, value };
};

export const letToCombination = (exp: Cons<Value>): Value => {
  const bindings: LetBinding[] = [];
  let cursor = exp.tail._tag === "Cons" ? exp.tail.head : nil;
  while (cursor._tag === "Cons") {
    const parsed = parseBinding(cursor.head);
    if (parsed !== undefined) {
      bindings.push(parsed);
    }
    cursor = cursor.tail;
  }
  const parameters = fromValues(bindings.map((binding) => binding.name));
  const argumentValues = fromValues(bindings.map((binding) => binding.value));
  const body = exp.tail._tag === "Cons" ? exp.tail.tail : nil;
  return list(makeLambda(parameters, body), ...toArray(argumentValues));
};

// (c) the scan installed in make-procedure ---------------------------

/** The book's question: scan in make-procedure or in procedure-body?
 * make-procedure is the right point: the rewrite happens once when the
 * closure is created, instead of on every application that reads the
 * body. */
export const makeProcedureScanned = (
  parameters: List<Value>,
  body: List<Value>,
  env: Env,
): CompoundProc => makeProcedure(parameters, scanOutDefines(body), env);

const fromValues = (items: ReadonlyArray<Value>): List<Value> =>
  items.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);

// A complete dispatch: the module's evaluate with a let clause, the
// scanned lookup, and the scanned make-procedure.

const evalIfScanned = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateScanned(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate)
      ? evaluateScanned(ifConsequent(exp), env)
      : evaluateScanned(ifAlternative(exp), env),
  );

const evalSequenceScanned = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<Value, EvaluationError> => {
  if (exps._tag === "Nil") {
    return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
  }
  if (exps.tail._tag === "Nil") {
    return evaluateScanned(exps.head, env);
  }
  return Effect.flatMap(evaluateScanned(exps.head, env), () => evalSequenceScanned(exps.tail, env));
};

const evalAssignmentScanned = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateScanned(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionScanned = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateScanned(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

const listOfValuesScanned = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> => {
  if (exps._tag === "Nil") {
    return Effect.succeed(nil);
  }
  return Effect.flatMap(evaluateScanned(exps.head, env), (first) =>
    Effect.map(listOfValuesScanned(exps.tail, env), (rest) => cons(first, rest)),
  );
};

const applyScanned = (
  procedure: Value,
  args: List<Value>,
): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag === "Primitive") {
    return applyPrimitiveProcedure(procedure, args);
  }
  if (procedure._tag === "Compound") {
    return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (callEnv) =>
      evalSequenceScanned(procedure.body, callEnv),
    );
  }
  return Effect.fail(new NotAProcedure({ value: format(procedure) }));
};

const evalApplicationScanned = (
  exp: Cons<Value>,
  env: Env,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateScanned(operator(exp), env), (procedure) =>
    Effect.flatMap(listOfValuesScanned(operands(exp), env), (args) =>
      applyScanned(procedure, args),
    ),
  );

export const evaluateScanned: Evaluate = (exp, env) => {
  if (isLet(exp)) {
    return evaluateScanned(letToCombination(exp), env);
  }
  if (isSelfEvaluating(exp)) {
    return Effect.succeed(exp);
  }
  if (isVariable(exp)) {
    return lookupVariableValueScanned(exp, env);
  }
  if (isQuoted(exp)) {
    return Effect.succeed(textOfQuotation(exp));
  }
  if (isAssignment(exp)) {
    return evalAssignmentScanned(exp, env);
  }
  if (isDefinition(exp)) {
    return evalDefinitionScanned(exp, env);
  }
  if (isIf(exp)) {
    return evalIfScanned(exp, env);
  }
  if (isLambda(exp)) {
    return Effect.succeed(makeProcedureScanned(lambdaParameters(exp), lambdaBody(exp), env));
  }
  if (isBegin(exp)) {
    return evalSequenceScanned(beginActions(exp), env);
  }
  if (isCond(exp)) {
    return evaluateScanned(condToIf(exp), env);
  }
  if (isApplication(exp)) {
    return evalApplicationScanned(exp, env);
  }
  return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
};

/** Reads one form and evaluates it with the scan installed. */
export const evalStringScanned = (text: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(
    Effect.try({
      try: () => read(text),
      catch: (error) =>
        new RuntimeError({
          message: "read failed",
          detail:
            error instanceof ReadError || error instanceof Error ? error.message : String(error),
        }),
    }),
    (exp) => evaluateScanned(exp, env),
  );

/** The book's f, whose two internal defines are mutually recursive. */
export const fDefinition =
  "(define (f x) (define (even? n) (if (= n 0) true (odd? (- n 1)))) (define (odd? n) (if (= n 0) false (even? (- n 1)))) (even? x))";

export function ex_4_16(): string {
  return "Internal defines become a let of '*unassigned*' bindings followed by set!s: scan-out-defines collects each define's name, pre-binds every name to the marker, and replaces the defines with assignments at the front of the body. Lookup is changed to fail when it finds the marker, so reading a name before its set! is an error instead of a silent wrong value. The scan belongs in make-procedure, not procedure-body: installed there the rewrite runs once when a closure is created, rather than on every application that reads the body.";
}
