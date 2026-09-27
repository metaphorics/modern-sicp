// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.3: rewrite eval so the dispatch on special forms is done in
 * data-directed style, as in the data-directed differentiation of exercise
 * 2.73: a table keyed by the operator symbol name (the car of the compound
 * expression is its type), one operation installed per form with put and
 * fetched with get. Self-evaluating expressions and variables carry no
 * type tag, so they bypass the table; a pair whose car is not installed is
 * a procedure application. The table is built once per environment,
 * because each installed operation wraps the module's helpers for the
 * frame it evaluates in.
 */
import { Effect, Option } from "effect";

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
  firstExp,
  firstOperand,
  ifAlternative,
  ifConsequent,
  ifPredicate,
  isApplication,
  isLastExp,
  isPair,
  isSelfEvaluating,
  isSymbol,
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
import { OpTable } from "../../packages/ch4/src/primitives.js";
import { format } from "../../packages/ch4/src/read.js";

/** Runs a table operation on the expression it guards out of the stored
 * Primitive argument list. */
const onExpression = (
  expList: List<Value>,
  run: (exp: Cons<Value>) => Effect.Effect<Value, EvaluationError>,
): Effect.Effect<Value, EvaluationError> =>
  expList._tag === "Cons"
    ? run(expList)
    : Effect.fail(new UnknownSyntax({ expr: format(expList) }));

/** The data-directed table for one environment: quote, if, set!, define,
 * lambda, begin, and cond, one operation each. */
const formTableFor = (env: Env): OpTable => {
  const table = new OpTable();
  table.put("quote", (expList) =>
    onExpression(expList, (exp) => Effect.succeed(textOfQuotation(exp))),
  );
  table.put("set!", (expList) => onExpression(expList, (exp) => evalAssignmentData(exp, env)));
  table.put("define", (expList) => onExpression(expList, (exp) => evalDefinitionData(exp, env)));
  table.put("if", (expList) => onExpression(expList, (exp) => evalIfData(exp, env)));
  table.put("lambda", (expList) =>
    onExpression(expList, (exp) =>
      Effect.succeed(makeProcedure(lambdaParameters(exp), lambdaBody(exp), env)),
    ),
  );
  table.put("begin", (expList) =>
    onExpression(expList, (exp) => evalSequenceData(beginActions(exp), env)),
  );
  table.put("cond", (expList) =>
    onExpression(expList, (exp) => evalDataDirected(condToIf(exp), env)),
  );
  return table;
};

const tables = new WeakMap<Env, OpTable>();

const formTable = (env: Env): OpTable => {
  const cached = tables.get(env);
  if (cached !== undefined) {
    return cached;
  }
  const table = formTableFor(env);
  tables.set(env, table);
  return table;
};

const taggedForm = (exp: Value): { tag: string; form: Cons<Value> } | undefined => {
  if (isPair(exp) && isSymbol(exp.head)) {
    return { tag: exp.head.name, form: exp };
  }
  return undefined;
};

/** The data-directed eval: a table hit runs the installed operation, a
 * pair with no installed operation is an application, the rest fails. */
export const evalDataDirected: Evaluate = (exp, env) => {
  if (isSelfEvaluating(exp)) {
    return Effect.succeed(exp);
  }
  if (isVariable(exp)) {
    return lookupVariableValue(exp, env);
  }
  const form = taggedForm(exp);
  if (form !== undefined) {
    const handler = formTable(env).get(form.tag);
    if (Option.isSome(handler)) {
      return handler.value(form.form);
    }
  }
  if (isApplication(exp)) {
    return Effect.flatMap(evalDataDirected(operator(exp), env), (procedure) =>
      Effect.flatMap(listOfValuesData(operands(exp), env), (args) => applyData(procedure, args)),
    );
  }
  return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
};

const evalSequenceData = (seq: List<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
  if (seq._tag === "Nil") {
    return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
  }
  if (isLastExp(seq)) {
    return evalDataDirected(firstExp(seq), env);
  }
  return Effect.flatMap(evalDataDirected(firstExp(seq), env), () =>
    evalSequenceData(restExps(seq), env),
  );
};

const applyData = (procedure: Value, args: List<Value>): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag === "Primitive") {
    return applyPrimitiveProcedure(procedure, args);
  }
  if (procedure._tag === "Compound") {
    return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (newEnv) =>
      evalSequenceData(procedure.body, newEnv),
    );
  }
  return Effect.fail(new NotAProcedure({ value: format(procedure) }));
};

const listOfValuesData = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> =>
  noOperands(exps)
    ? Effect.succeed(nil)
    : Effect.flatMap(evalDataDirected(firstOperand(exps), env), (first) =>
        Effect.map(listOfValuesData(restOperands(exps), env), (rest) => cons(first, rest)),
      );

const evalIfData = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalDataDirected(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate)
      ? evalDataDirected(ifConsequent(exp), env)
      : evalDataDirected(ifAlternative(exp), env),
  );

const evalAssignmentData = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalDataDirected(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionData = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evalDataDirected(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

export function ex_4_03(): string {
  return (
    "The dispatch is a table keyed by the form's tag: quote, if, set!, define, lambda, begin, " +
    "and cond are installed as one operation each, fetched by the car of the expression; " +
    "self-evaluating expressions and variables bypass the table because they carry no tag, " +
    "and a pair whose car is not installed falls through to the ordinary application path. " +
    "Adding a new special form is now one more put, not one more cond clause."
  );
}
