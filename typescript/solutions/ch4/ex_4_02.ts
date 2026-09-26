// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.2: (a) Louis Reasoner would move the clause for procedure
 * applications in front of the clause for assignments. That cannot work:
 * a definition such as (define x 3) is itself a pair, so it reaches the
 * application clause and the evaluator tries to apply the symbol define,
 * which is unbound. `evalApplicationsFirst` reproduces the failure on the
 * real evaluator data. (b) With call-tagged applications the collision
 * disappears: a procedure application must begin with the keyword call,
 * as in (call + 1 2). `evalCallTagged` is the full dispatch of that
 * dialect, with no generic application clause outside call.
 */
import { Effect } from "effect";

import {
  applyPrimitiveProcedure,
  applyProcedure,
  assignmentValue,
  assignmentVariable,
  beginActions,
  condToIf,
  defineVariableValue,
  definitionValue,
  definitionVariable,
  evalAssignment,
  evalDefinition,
  evalIf,
  evalSequence,
  evaluate,
  extendEnvironment,
  firstExp,
  firstOperand,
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
  isLastExp,
  isPair,
  isQuoted,
  isSelfEvaluating,
  isSymbol,
  isTrue,
  isVariable,
  lambdaBody,
  lambdaParameters,
  listOfValues,
  lookupVariableValue,
  makeProcedure,
  noOperands,
  ok,
  operands,
  operator,
  restExps,
  restOperands,
  setVariableValue,
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

/** Louis's dispatch: the application clause first. Every pair now looks
 * like an application, special forms included. */
export const evalApplicationsFirst: Evaluate = (exp, env) => {
  if (isSelfEvaluating(exp)) {
    return Effect.succeed(exp);
  }
  if (isVariable(exp)) {
    return lookupVariableValue(exp, env);
  }
  if (isQuoted(exp)) {
    return Effect.succeed(textOfQuotation(exp));
  }
  if (isApplication(exp)) {
    return Effect.flatMap(evaluate(operator(exp), env), (procedure) =>
      Effect.flatMap(listOfValues(operands(exp), env), (args) => applyProcedure(procedure, args)),
    );
  }
  if (isAssignment(exp)) {
    return evalAssignment(exp, env);
  }
  if (isDefinition(exp)) {
    return evalDefinition(exp, env);
  }
  if (isIf(exp)) {
    return evalIf(exp, env);
  }
  if (isLambda(exp)) {
    return Effect.succeed(makeProcedure(lambdaParameters(exp), lambdaBody(exp), env));
  }
  if (isBegin(exp)) {
    return evalSequence(beginActions(exp), env);
  }
  if (isCond(exp)) {
    return evalApplicationsFirst(condToIf(exp), env);
  }
  return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
};

const isCallForm = (exp: Value): exp is Cons<Value> =>
  isPair(exp) && isSymbol(exp.head) && exp.head.name === "call";

const callOperator = (exp: Cons<Value>): Value => {
  const rest = exp.tail;
  return rest._tag === "Cons" ? rest.head : nil;
};

const callOperands = (exp: Cons<Value>): List<Value> =>
  exp.tail._tag === "Cons" ? exp.tail.tail : nil;

export const evalCallTagged: Evaluate = (exp, env) => {
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
    return evalAssignmentCall(exp, env);
  }
  if (isDefinition(exp)) {
    return evalDefinitionCall(exp, env);
  }
  if (isIf(exp)) {
    return evalIfCall(exp, env);
  }
  if (isLambda(exp)) {
    return Effect.succeed(makeProcedure(lambdaParameters(exp), lambdaBody(exp), env));
  }
  if (isBegin(exp)) {
    return evalSequenceCall(beginActions(exp), env);
  }
  if (isCond(exp)) {
    return evalCallTagged(condToIf(exp), env);
  }
  if (isCallForm(exp)) {
    return Effect.flatMap(evalCallTagged(callOperator(exp), env), (procedure) =>
      Effect.flatMap(listOfValuesCall(callOperands(exp), env), (args) =>
        applyCall(procedure, args),
      ),
    );
  }
  return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
};

const evalSequenceCall = (seq: List<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
  if (seq._tag === "Nil") {
    return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
  }
  if (isLastExp(seq)) {
    return evalCallTagged(firstExp(seq), env);
  }
  return Effect.flatMap(evalCallTagged(firstExp(seq), env), () =>
    evalSequenceCall(restExps(seq), env),
  );
};

const applyCall = (procedure: Value, args: List<Value>): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag === "Primitive") {
    return applyPrimitiveProcedure(procedure, args);
  }
  if (procedure._tag === "Compound") {
    return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (newEnv) =>
      evalSequenceCall(procedure.body, newEnv),
    );
  }
  return Effect.fail(new NotAProcedure({ value: format(procedure) }));
};

const listOfValuesCall = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> =>
  noOperands(exps)
    ? Effect.succeed(nil)
    : Effect.flatMap(evalCallTagged(firstOperand(exps), env), (first) =>
        Effect.map(listOfValuesCall(restOperands(exps), env), (rest) => cons(first, rest)),
      );

const evalIfCall = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalCallTagged(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate)
      ? evalCallTagged(ifConsequent(exp), env)
      : evalCallTagged(ifAlternative(exp), env),
  );

const evalAssignmentCall = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalCallTagged(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionCall = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalCallTagged(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

export function ex_4_02(): string {
  return (
    "(a) With the application clause first, (define x 3) is a pair, so it is taken for an " +
    "application: the evaluator evaluates the operator, the unbound symbol define, and fails " +
    "with UnboundVariable instead of ever binding x. (b) Requiring every application to begin " +
    "with call removes the collision: (call + 1 2) evaluates to 3, while (+ 1 2) is no longer " +
    "valid syntax and fails with UnknownSyntax."
  );
}
