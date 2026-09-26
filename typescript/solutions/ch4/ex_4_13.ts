// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.13: make-unbound!. The form (make-unbound! x) removes x's
 * binding from exactly the first frame of the chain that has one; a name
 * bound in no frame fails with UnboundVariable. The dispatch below is
 * complete, so an unbind works inside procedure bodies, where the inner
 * frame is the call frame.
 */
import { Effect, HashMap, Option, Ref } from "effect";

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
  makeProcedure,
  ok,
  operands,
  operator,
  setVariableValue,
  symbol,
  taggedList,
  textOfQuotation,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Evaluate, SymbolValue, Value } from "../../packages/ch4/src/core.js";
import {
  type EvaluationError,
  NotAProcedure,
  RuntimeError,
  UnboundVariable,
  UnknownSyntax,
} from "../../packages/ch4/src/errors.js";
import { type Cons, cons, type List, nil } from "../../packages/ch4/src/list.js";
import { format, ReadError, read } from "../../packages/ch4/src/read.js";

export const isMakeUnbound = (exp: Value): exp is Cons<Value> => taggedList("make-unbound!", exp);

export const unboundTarget = (exp: Cons<Value>): SymbolValue => {
  const target = exp.tail._tag === "Cons" ? exp.tail.head : symbol("<malformed>");
  return target._tag === "Symbol" ? target : symbol("<malformed>");
};

const removeBinding = (env: Env, name: string): Effect.Effect<boolean> =>
  Effect.flatMap(Ref.get(env.vars), (vars) =>
    HashMap.has(vars, name)
      ? Effect.map(
          Ref.update(env.vars, (current) => HashMap.remove(current, name)),
          () => true,
        )
      : Effect.succeed(false),
  );

const unbindInChain = (env: Env, name: string): Effect.Effect<boolean> => {
  const parent = env.parent;
  return Effect.flatMap(removeBinding(env, name), (removed) =>
    removed || Option.isNone(parent) ? Effect.succeed(removed) : unbindInChain(parent.value, name),
  );
};

/** The chosen specification: remove the binding only in the first frame
 * that has it; a name bound nowhere is an error. */
export const makeUnbound = (
  variable: SymbolValue,
  env: Env,
): Effect.Effect<void, EvaluationError> =>
  Effect.flatMap(unbindInChain(env, variable.name), (removed) =>
    removed
      ? Effect.asVoid(Effect.void)
      : Effect.fail(new UnboundVariable({ name: variable.name })),
  );

// A complete dispatch: the module's evaluate with the make-unbound!
// clause added, so unbinding works in nested bodies too.

const evalIfUnbound = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateWithUnbound(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate)
      ? evaluateWithUnbound(ifConsequent(exp), env)
      : evaluateWithUnbound(ifAlternative(exp), env),
  );

const evalSequenceUnbound = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<Value, EvaluationError> => {
  if (exps._tag === "Nil") {
    return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
  }
  if (exps.tail._tag === "Nil") {
    return evaluateWithUnbound(exps.head, env);
  }
  return Effect.flatMap(evaluateWithUnbound(exps.head, env), () =>
    evalSequenceUnbound(exps.tail, env),
  );
};

const evalAssignmentUnbound = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateWithUnbound(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionUnbound = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateWithUnbound(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

const listOfValuesUnbound = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> => {
  if (exps._tag === "Nil") {
    return Effect.succeed(nil);
  }
  return Effect.flatMap(evaluateWithUnbound(exps.head, env), (first) =>
    Effect.map(listOfValuesUnbound(exps.tail, env), (rest) => cons(first, rest)),
  );
};

const applyUnbound = (
  procedure: Value,
  args: List<Value>,
): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag === "Primitive") {
    return applyPrimitiveProcedure(procedure, args);
  }
  if (procedure._tag === "Compound") {
    return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (callEnv) =>
      evalSequenceUnbound(procedure.body, callEnv),
    );
  }
  return Effect.fail(new NotAProcedure({ value: format(procedure) }));
};

const evalApplicationUnbound = (
  exp: Cons<Value>,
  env: Env,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateWithUnbound(operator(exp), env), (procedure) =>
    Effect.flatMap(listOfValuesUnbound(operands(exp), env), (args) =>
      applyUnbound(procedure, args),
    ),
  );

export const evaluateWithUnbound: Evaluate = (exp, env) => {
  if (isMakeUnbound(exp)) {
    return Effect.map(makeUnbound(unboundTarget(exp), env), () => ok);
  }
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
    return evalAssignmentUnbound(exp, env);
  }
  if (isDefinition(exp)) {
    return evalDefinitionUnbound(exp, env);
  }
  if (isIf(exp)) {
    return evalIfUnbound(exp, env);
  }
  if (isLambda(exp)) {
    return Effect.succeed(makeProcedure(lambdaParameters(exp), lambdaBody(exp), env));
  }
  if (isBegin(exp)) {
    return evalSequenceUnbound(beginActions(exp), env);
  }
  if (isCond(exp)) {
    return evaluateWithUnbound(condToIf(exp), env);
  }
  if (isApplication(exp)) {
    return evalApplicationUnbound(exp, env);
  }
  return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
};

/** Reads one form and evaluates it with make-unbound! installed. */
export const evalStringWithUnbound = (
  text: string,
  env: Env,
): Effect.Effect<Value, EvaluationError> =>
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
    (exp) => evaluateWithUnbound(exp, env),
  );

export function ex_4_13(): string {
  return "make-unbound! is specified to remove the binding from exactly the first frame of the chain that has one, and to fail with an unbound-variable error when no frame has it. That keeps unbinding local: an inner binding can be removed to expose the outer one again, while the outer frames are untouched. Unbinding a name that is bound nowhere is an error, because silently ignoring it would hide real mistakes.";
}
