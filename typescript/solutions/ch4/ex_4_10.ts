// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.10: eval is independent of the surface syntax once the
 * special forms live in a syntax table. This file parameterizes the
 * evaluator over a table mapping each special-form tag to a handler, then
 * runs the same evaluator under two syntaxes: the standard tags, and the
 * same language with every tag spelled backwards. Nested forms and
 * procedure bodies follow the syntax of the evaluator that runs them.
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
  textOfQuotation,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Evaluate, Value } from "../../packages/ch4/src/core.js";
import {
  type EvaluationError,
  NotAProcedure,
  RuntimeError,
  UnknownSyntax,
} from "../../packages/ch4/src/errors.js";
import { type Cons, cons, type List, nil } from "../../packages/ch4/src/list.js";
import { format, ReadError, read } from "../../packages/ch4/src/read.js";

/** One special form: given the tagged expression, the environment, and
 * the evaluator of the running syntax, produce the value. */
export type SyntaxHandler = (
  exp: Cons<Value>,
  env: Env,
  evalExp: Evaluate,
) => Effect.Effect<Value, EvaluationError>;

/** The book's syntax abstraction: tag text to handler. */
export type SyntaxTable = ReadonlyMap<string, SyntaxHandler>;

const evalQuoteIn: SyntaxHandler = (exp) => Effect.succeed(textOfQuotation(exp));

const evalIfIn: SyntaxHandler = (exp, env, evalExp) =>
  Effect.flatMap(evalExp(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate) ? evalExp(ifConsequent(exp), env) : evalExp(ifAlternative(exp), env),
  );

const evalAssignmentIn: SyntaxHandler = (exp, env, evalExp) =>
  Effect.flatMap(evalExp(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionIn: SyntaxHandler = (exp, env, evalExp) =>
  Effect.flatMap(evalExp(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

const evalLambdaIn: SyntaxHandler = (exp, env) =>
  Effect.succeed(makeProcedure(lambdaParameters(exp), lambdaBody(exp), env));

const sequenceIn = (
  exps: List<Value>,
  env: Env,
  evalExp: Evaluate,
): Effect.Effect<Value, EvaluationError> => {
  if (exps._tag === "Nil") {
    return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
  }
  if (exps.tail._tag === "Nil") {
    return evalExp(exps.head, env);
  }
  return Effect.flatMap(evalExp(exps.head, env), () => sequenceIn(exps.tail, env, evalExp));
};

const evalBeginIn: SyntaxHandler = (exp, env, evalExp) =>
  sequenceIn(beginActions(exp), env, evalExp);

const evalCondIn: SyntaxHandler = (exp, env, evalExp) => evalExp(condToIf(exp), env);

const listOfValuesIn = (
  exps: List<Value>,
  env: Env,
  evalExp: Evaluate,
): Effect.Effect<List<Value>, EvaluationError> => {
  if (exps._tag === "Nil") {
    return Effect.succeed(nil);
  }
  return Effect.flatMap(evalExp(exps.head, env), (first) =>
    Effect.map(listOfValuesIn(exps.tail, env, evalExp), (rest) => cons(first, rest)),
  );
};

const applyIn = (
  procedure: Value,
  args: List<Value>,
  evalExp: Evaluate,
): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag === "Primitive") {
    return applyPrimitiveProcedure(procedure, args);
  }
  if (procedure._tag === "Compound") {
    return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (callEnv) =>
      sequenceIn(procedure.body, callEnv, evalExp),
    );
  }
  return Effect.fail(new NotAProcedure({ value: format(procedure) }));
};

/** The evaluator parameterized by a syntax table: a tagged head whose tag
 * is installed dispatches to its handler, any other combination is an
 * application, and the same table rules every nested evaluation. */
export const evaluateIn = (syntax: SyntaxTable): Evaluate => {
  const evalExp: Evaluate = (exp, env) => {
    if (isSelfEvaluating(exp)) {
      return Effect.succeed(exp);
    }
    if (isVariable(exp)) {
      return lookupVariableValue(exp, env);
    }
    if (exp._tag === "Cons") {
      const head = exp.head;
      if (head._tag === "Symbol") {
        const handler = syntax.get(head.name);
        if (handler !== undefined) {
          return handler(exp, env, evalExp);
        }
      }
      return Effect.flatMap(evalExp(operator(exp), env), (procedure) =>
        Effect.flatMap(listOfValuesIn(operands(exp), env, evalExp), (args) =>
          applyIn(procedure, args, evalExp),
        ),
      );
    }
    return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
  };
  return evalExp;
};

const table = (entries: ReadonlyArray<readonly [string, SyntaxHandler]>): SyntaxTable =>
  new Map(entries);

/** The standard spelling. */
export const makeSchemeTable = (): SyntaxTable =>
  table([
    ["quote", evalQuoteIn],
    ["if", evalIfIn],
    ["set!", evalAssignmentIn],
    ["define", evalDefinitionIn],
    ["lambda", evalLambdaIn],
    ["begin", evalBeginIn],
    ["cond", evalCondIn],
  ]);

/** The same seven forms, every tag spelled backwards. */
export const makeBackwardsTable = (): SyntaxTable =>
  table([
    ["etouq", evalQuoteIn],
    ["fi", evalIfIn],
    ["!tes", evalAssignmentIn],
    ["enifed", evalDefinitionIn],
    ["adbmal", evalLambdaIn],
    ["nigeb", evalBeginIn],
    ["dnoc", evalCondIn],
  ]);

/** Reads one form and evaluates it under the given syntax. */
export const evalStringIn = (
  syntax: SyntaxTable,
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
    (exp) => evaluateIn(syntax)(exp, env),
  );

export function ex_4_10(): string {
  return "Once the special forms live in a syntax table, eval never mentions a tag: the same evaluator runs any syntax you install. This file installs two tables, the standard tags and the same forms spelled backwards, and one program, (define square (lambda (x) (* x x))) with (square 7), evaluates to 49 under both; a tag from the other table is just an unbound variable.";
}
