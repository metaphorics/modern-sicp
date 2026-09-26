// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.18: the alternative scan-out strategy. Each define's value
 * expression is computed inside one inner let before any set! runs, while
 * the text's scan runs the set!s in order. The tests pin a program whose
 * value expression reads a sibling: the alternative fails on it and the
 * text's scan works, and a program whose sibling read is deferred through a
 * closure defined earlier works under both.
 *
 * The book spells both scans with lets; this edition's evaluator has no let
 * clause, so both scans emit the lets' own derivations, lambda
 * applications, directly. The dispatch is complete rather than delegating
 * to the module's eval helpers, because those recurse through the module's
 * evaluate and would leave nested lambdas unscanned.
 */
import { Effect } from "effect";
import {
  applyProcedure,
  assignmentValue,
  assignmentVariable,
  beginActions,
  condToIf,
  defineVariableValue,
  definitionValue,
  definitionVariable,
  extendEnvironment,
  firstExp,
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
  restExps,
  setupEnvironment,
  setVariableValue,
  symbol,
  textOfQuotation,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Evaluate, SymbolValue, Value } from "../../packages/ch4/src/core.js";
import {
  type EvaluationError,
  RuntimeError,
  UnknownSyntax,
} from "../../packages/ch4/src/errors.js";
import { cons, type List, nil, toArray } from "../../packages/ch4/src/list.js";
import { format, read } from "../../packages/ch4/src/read.js";

const unassignedName = "*unassigned*";

const unassigned = (): Value => symbol(unassignedName);

const isUnassigned = (value: Value): boolean =>
  value._tag === "Symbol" && value.name === unassignedName;

/** One scanned-out internal definition. */
interface ScannedDefine {
  readonly name: SymbolValue;
  readonly value: Value;
}

const listOf = (items: ReadonlyArray<Value>): List<Value> =>
  items.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);

const list = (...items: Value[]): List<Value> => listOf(items);

const concatLists = (lists: ReadonlyArray<List<Value>>): List<Value> =>
  listOf(lists.flatMap((items) => toArray(items)));

interface ScannedBody {
  readonly defines: ReadonlyArray<ScannedDefine>;
  readonly rest: List<Value>;
}

/** Takes the leading defines of a body, as both scans do. */
const takeDefines = (body: List<Value>): ScannedBody => {
  const defines: ScannedDefine[] = [];
  let rest: List<Value> = body;
  while (rest._tag === "Cons" && isDefinition(rest.head)) {
    defines.push({ name: definitionVariable(rest.head), value: definitionValue(rest.head) });
    rest = rest.tail;
  }
  return { defines, rest };
};

/**
 * The text's scan: one lambda application pre-binds every name to
 * '*unassigned*' and then runs the set!s in order.
 */
export const scanOutDefinesText = (body: List<Value>): List<Value> => {
  const { defines, rest } = takeDefines(body);
  if (defines.length === 0) {
    return body;
  }
  const lambda = cons<Value>(
    symbol("lambda"),
    cons(
      listOf(defines.map((d) => d.name)),
      concatLists([listOf(defines.map((d) => list(symbol("set!"), d.name, d.value))), rest]),
    ),
  );
  const quotedUnassigned = list(symbol("quote"), unassigned());
  return cons<Value>(cons(lambda, listOf(defines.map(() => quotedUnassigned))), nil);
};

/**
 * The alternative scan: every value expression is computed in one inner let
 * before any set! runs, then the results are assigned.
 */
export const scanOutDefinesAlternative = (body: List<Value>): List<Value> => {
  const { defines, rest } = takeDefines(body);
  if (defines.length === 0) {
    return body;
  }
  const tempNames: SymbolValue[] = [];
  const setForms: Value[] = [];
  defines.forEach((d, i) => {
    const temp = symbol(`%value-${i + 1}`);
    tempNames.push(temp);
    setForms.push(list(symbol("set!"), d.name, temp));
  });
  const innerLambda = cons<Value>(symbol("lambda"), cons(listOf(tempNames), listOf(setForms)));
  const innerApplication = cons<Value>(innerLambda, listOf(defines.map((d) => d.value)));
  const outerLambda = cons<Value>(
    symbol("lambda"),
    cons(listOf(defines.map((d) => d.name)), cons(innerApplication, rest)),
  );
  const quotedUnassigned = list(symbol("quote"), unassigned());
  return cons<Value>(cons(outerLambda, listOf(defines.map(() => quotedUnassigned))), nil);
};

/** The book's lookup plus the *unassigned* guard both scans rely on. */
const lookupChecked = (variable: SymbolValue, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(lookupVariableValue(variable, env), (value) =>
    isUnassigned(value)
      ? Effect.fail(
          new RuntimeError({
            message: `variable used before assignment: ${variable.name}`,
            detail: unassignedName,
          }),
        )
      : Effect.succeed(value),
  );

const evalSequenceWith = (
  evaluate: Evaluate,
  exps: List<Value>,
  env: Env,
): Effect.Effect<Value, EvaluationError> => {
  if (exps._tag === "Nil") {
    return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
  }
  if (isLastExp(exps)) {
    return evaluate(firstExp(exps), env);
  }
  return Effect.flatMap(evaluate(firstExp(exps), env), () =>
    evalSequenceWith(evaluate, restExps(exps), env),
  );
};

/** One dispatch shared by both scans; only the body transform differs. */
const evaluatorWith = (scanBody: (body: List<Value>) => List<Value>): Evaluate => {
  const evaluate: Evaluate = (exp, env) => {
    if (isSelfEvaluating(exp)) {
      return Effect.succeed(exp);
    }
    if (isVariable(exp)) {
      return lookupChecked(exp, env);
    }
    if (isQuoted(exp)) {
      return Effect.succeed(textOfQuotation(exp));
    }
    if (isAssignment(exp)) {
      return Effect.flatMap(evaluate(assignmentValue(exp), env), (value) =>
        Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
      );
    }
    if (isDefinition(exp)) {
      return Effect.flatMap(evaluate(definitionValue(exp), env), (value) =>
        Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
      );
    }
    if (isIf(exp)) {
      return Effect.flatMap(evaluate(ifPredicate(exp), env), (predicate) =>
        isTrue(predicate) ? evaluate(ifConsequent(exp), env) : evaluate(ifAlternative(exp), env),
      );
    }
    if (isLambda(exp)) {
      return Effect.succeed(makeProcedure(lambdaParameters(exp), scanBody(lambdaBody(exp)), env));
    }
    if (isBegin(exp)) {
      return evalSequenceFrom(beginActions(exp), env);
    }
    if (isCond(exp)) {
      return evaluate(condToIf(exp), env);
    }
    if (isApplication(exp)) {
      return Effect.flatMap(evaluate(operator(exp), env), (procedure) =>
        Effect.flatMap(listOfValuesFrom(operands(exp), env), (args) => {
          if (procedure._tag !== "Compound") {
            return applyProcedure(procedure, args);
          }
          return Effect.flatMap(
            extendEnvironment(procedure.params, args, procedure.env),
            (newEnv) => evalSequenceFrom(procedure.body, newEnv),
          );
        }),
      );
    }
    return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
  };
  const listOfValuesFrom = (
    exps: List<Value>,
    env: Env,
  ): Effect.Effect<List<Value>, EvaluationError> => {
    if (exps._tag === "Nil") {
      return Effect.succeed(nil);
    }
    return Effect.flatMap(evaluate(exps.head, env), (first) =>
      Effect.map(listOfValuesFrom(exps.tail, env), (rest) => cons(first, rest)),
    );
  };
  const evalSequenceFrom = (exps: List<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
    evalSequenceWith(evaluate, exps, env);
  return evaluate;
};

/** Internal definitions interpreted with the text's scan. */
export const evaluateText: Evaluate = evaluatorWith(scanOutDefinesText);

/** Internal definitions interpreted with the alternative scan. */
export const evaluateAlternative: Evaluate = evaluatorWith(scanOutDefinesAlternative);

/** Defines the given program in a fresh global environment. */
export const defineUnder = (
  evaluate: Evaluate,
  source: string,
): Effect.Effect<Env, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) => Effect.map(evaluate(read(source), env), () => env));

export function ex_4_18(): string {
  return (
    "The solve procedure fails under the alternative scan and works under the text's. " +
    "The alternative computes every value expression before any set! runs, and dy's value " +
    "expression (stream-map f y) reads y immediately for the stream's first element, while " +
    "y is still *unassigned*; the text's scan has already run (set! y ...) when dy's value " +
    "expression is evaluated. The read must therefore not happen while the defines are " +
    "being evaluated, which is exactly the restriction the alternative enforces and solve " +
    "violates. When the sibling read is deferred through a closure that is only called " +
    "after all the set!s, both scans give the same answer."
  );
}
