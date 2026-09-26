// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.5: cond's additional clause syntax (test => recipient). If
 * test evaluates to something other than false, recipient is evaluated;
 * its value must be a procedure of one argument, and that procedure is
 * then invoked on the value of test. The expansion below follows the
 * module's expand-clauses shape, and an arrow clause becomes a lambda
 * application that binds the test value once, so the test is never
 * evaluated twice.
 */
import { Effect } from "effect";

import {
  applyPrimitiveProcedure,
  assignmentValue,
  assignmentVariable,
  beginActions,
  condActions,
  condClauses,
  condPredicate,
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
  isCond,
  isCondElseClause,
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
  makeIf,
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

const isArrowClause = (clause: Value): boolean => {
  if (!isPair(clause)) {
    return false;
  }
  const marker = clause.tail;
  return marker._tag === "Cons" && isSymbolName(marker.head, "=>") && marker.tail._tag === "Cons";
};

const isSymbolName = (value: Value, name: string): boolean =>
  value._tag === "Symbol" && value.name === name;

const clauseRecipient = (clause: Cons<Value>): Value => {
  const afterMarker = clause.tail;
  return afterMarker._tag === "Cons" && afterMarker.tail._tag === "Cons"
    ? afterMarker.tail.head
    : nil;
};

const expandClausesWithArrow = (clauses: List<Value>): Value => {
  if (clauses._tag === "Nil") {
    return falseValue;
  }
  const first = clauses.head;
  if (!isPair(first)) {
    return falseValue;
  }
  if (isCondElseClause(first)) {
    return sequenceToExp(condActions(first));
  }
  if (isArrowClause(first)) {
    const bound = symbol("t");
    return cons(
      makeLambda(
        list(bound),
        cons(
          makeIf(
            bound,
            cons(clauseRecipient(first), cons(bound, nil)),
            expandClausesWithArrow(clauses.tail),
          ),
          nil,
        ),
      ),
      cons(condPredicate(first), nil),
    );
  }
  return makeIf(
    condPredicate(first),
    sequenceToExp(condActions(first)),
    expandClausesWithArrow(clauses.tail),
  );
};

/** The book's cond->if, extended with arrow clauses. */
export const condToIfWithArrow = (exp: Cons<Value>): Value =>
  expandClausesWithArrow(condClauses(exp));

export const evalWithArrowCond: Evaluate = (exp, env) => {
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
  if (isIf(exp)) {
    return evalIfLocal(exp, env);
  }
  if (isLambda(exp)) {
    return Effect.succeed(makeProcedure(lambdaParameters(exp), lambdaBody(exp), env));
  }
  if (isBegin(exp)) {
    return evalSequenceLocal(beginActions(exp), env);
  }
  if (isCond(exp)) {
    return evalWithArrowCond(condToIfWithArrow(exp), env);
  }
  if (isPair(exp)) {
    return Effect.flatMap(evalWithArrowCond(operator(exp), env), (procedure) =>
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
    return evalWithArrowCond(firstExp(seq), env);
  }
  return Effect.flatMap(evalWithArrowCond(firstExp(seq), env), () =>
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
    : Effect.flatMap(evalWithArrowCond(firstOperand(exps), env), (first) =>
        Effect.map(listOfValuesLocal(restOperands(exps), env), (rest) => cons(first, rest)),
      );

const evalIfLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithArrowCond(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate)
      ? evalWithArrowCond(ifConsequent(exp), env)
      : evalWithArrowCond(ifAlternative(exp), env),
  );

const evalAssignmentLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithArrowCond(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionLocal = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalWithArrowCond(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

export function ex_4_05(): string {
  return (
    "An arrow clause (test => recipient) expands like an ordinary clause except that the " +
    "consequent is a lambda application: the test value is bound to a parameter and the " +
    "recipient is called on it, so a true test is evaluated once and handed to the " +
    "recipient, and a false test moves on to the remaining clauses. With this expansion an " +
    "assoc-driven lookup defined in the evaluated language returns the matched record's " +
    "value, and missing keys still fall through to the else clause."
  );
}
