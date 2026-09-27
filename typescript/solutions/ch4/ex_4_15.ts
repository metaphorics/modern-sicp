// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.15: the halting diagonal, executed rather than argued. The
 * object language gets the book's run-forever and try; halts? is provided
 * by the host as an oracle with an adjustable answer. A step counter
 * wrapped around a complete dispatch turns "runs forever" into an
 * observable failure: past its fuel limit the evaluator fails with a
 * RuntimeError instead of hanging, so (try try) under the true oracle
 * exhausts its fuel, and (try (lambda (u) u)) under the false oracle
 * returns 'halted, the wrong answer.
 */
import { Effect, Ref } from "effect";

import {
  addBindingToFrame,
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
  setupEnvironment,
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
import { type Cons, cons, type List, nil, toArray } from "../../packages/ch4/src/list.js";
import { format, ReadError, read } from "../../packages/ch4/src/read.js";

/** The book's given definitions, spelled in the object language. */
export const RUN_FOREVER = "(define (run-forever) (run-forever))";
export const TRY = "(define (try p) (if (halts? p p) (run-forever) 'halted))";

/** A claimed decision procedure for halting, correct by assumption only. */
export interface HaltsOracle {
  readonly name: string;
  readonly decides: (procedure: Value, input: Value) => boolean;
}

export const trueOracle: HaltsOracle = { name: "always true", decides: () => true };
export const falseOracle: HaltsOracle = { name: "always false", decides: () => false };

export const installOracle = (env: Env, oracle: HaltsOracle): Effect.Effect<void> =>
  addBindingToFrame(
    symbol("halts?"),
    {
      _tag: "Primitive",
      name: "halts?",
      fn: (args) => {
        const items = toArray(args);
        const procedure = items[0] ?? nil;
        const input = items[1] ?? nil;
        return Effect.succeed({ _tag: "Boolean", b: oracle.decides(procedure, input) });
      },
    },
    env,
  );

export interface FueledEvaluator {
  readonly evaluate: Evaluate;
  readonly stepsUsed: Effect.Effect<number>;
  readonly limit: number;
}

/** A complete dispatch wrapped in a step counter: every dispatch spends
 * one unit of fuel, and one step past the limit fails with a RuntimeError
 * instead of diverging. */
export const makeFueledEvaluator = (limit: number): Effect.Effect<FueledEvaluator> =>
  Effect.map(Ref.make(0), (steps) => {
    const spend = (): Effect.Effect<void, EvaluationError> =>
      Effect.flatMap(
        Ref.updateAndGet(steps, (used) => used + 1),
        (used) =>
          used > limit
            ? Effect.fail(
                new RuntimeError({
                  message: "out of fuel",
                  detail: `${used} steps exceeds the limit of ${limit}`,
                }),
              )
            : Effect.asVoid(Effect.void),
      );

    const evalIf = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
      Effect.flatMap(evaluate(ifPredicate(exp), env), (predicate) =>
        isTrue(predicate) ? evaluate(ifConsequent(exp), env) : evaluate(ifAlternative(exp), env),
      );

    const evalSequence = (exps: List<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
      if (exps._tag === "Nil") {
        return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
      }
      if (exps.tail._tag === "Nil") {
        return evaluate(exps.head, env);
      }
      return Effect.flatMap(evaluate(exps.head, env), () => evalSequence(exps.tail, env));
    };

    const listOfValues = (
      exps: List<Value>,
      env: Env,
    ): Effect.Effect<List<Value>, EvaluationError> => {
      if (exps._tag === "Nil") {
        return Effect.succeed(nil);
      }
      return Effect.flatMap(evaluate(exps.head, env), (first) =>
        Effect.map(listOfValues(exps.tail, env), (rest) => cons(first, rest)),
      );
    };

    const applyFueled = (
      procedure: Value,
      args: List<Value>,
    ): Effect.Effect<Value, EvaluationError> => {
      if (procedure._tag === "Primitive") {
        return applyPrimitiveProcedure(procedure, args);
      }
      if (procedure._tag === "Compound") {
        return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (callEnv) =>
          evalSequence(procedure.body, callEnv),
        );
      }
      return Effect.fail(new NotAProcedure({ value: format(procedure) }));
    };

    const evalApplication = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
      Effect.flatMap(evaluate(operator(exp), env), (procedure) =>
        Effect.flatMap(listOfValues(operands(exp), env), (args) => applyFueled(procedure, args)),
      );

    const evalAssignment = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
      Effect.flatMap(evaluate(assignmentValue(exp), env), (value) =>
        Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
      );

    const evalDefinition = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
      Effect.flatMap(evaluate(definitionValue(exp), env), (value) =>
        Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
      );

    const evaluate: Evaluate = (exp, env) =>
      Effect.flatMap(spend(), () => {
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
          return evaluate(condToIf(exp), env);
        }
        if (isApplication(exp)) {
          return evalApplication(exp, env);
        }
        return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
      });

    return { evaluate, stepsUsed: Ref.get(steps), limit };
  });

/** Reads and evaluates each form in order with the fueled evaluator. */
export const runFueled = (
  fueled: FueledEvaluator,
  sources: ReadonlyArray<string>,
  env: Env,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(
    Effect.forEach(sources, (source) =>
      Effect.flatMap(
        Effect.try({
          try: () => read(source),
          catch: (error) =>
            new RuntimeError({
              message: "read failed",
              detail:
                error instanceof ReadError || error instanceof Error
                  ? error.message
                  : String(error),
            }),
        }),
        (exp) => fueled.evaluate(exp, env),
      ),
    ),
    (values) => Effect.succeed(values[values.length - 1] ?? nil),
  );

/** A global environment with the book's definitions and one oracle. */
export const makeDiagonalEnvironment = (oracle: HaltsOracle): Effect.Effect<Env, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) => Effect.map(installOracle(env, oracle), () => env));

export function ex_4_15(): string {
  return "halts? cannot exist: suppose it does and run-forever and try are defined as the book gives them. If halts? answers true for (try try), try calls run-forever and diverges, so the answer was false; if it answers false, (try try) halts with 'halted, so the answer was true. Both oracles here are run: the true one drives (try try) until the fuel-limited evaluator gives up, the false one answers 'halted for a procedure that halts on itself. Every answer is wrong, so no such decision procedure exists.";
}
