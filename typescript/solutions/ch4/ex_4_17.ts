// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.17: why scanning definitions out of a body adds one frame, and
 * how to get simultaneous scope without it.
 *
 * The text's scan wraps the set!s in a let, and a let is a lambda
 * application: every call opens a second frame whose only content is the
 * *unassigned* pre-bindings. The design answer scans into the SAME frame:
 * the application extends the environment once, with the parameters plus
 * the scanned names pre-bound to *unassigned*, and then runs the set!-only
 * body. The tests pin identical program behavior under the sequential,
 * scanned, and same-frame evaluators, and the frame counts that differ by
 * one between the two make-procedure variants.
 *
 * The book spells the scanned body with a let; this edition's evaluator has
 * no let clause, so the scan emits the let's own derivation, a lambda
 * application, directly. The observable structure is the same. The dispatch
 * here is complete rather than delegating to the module's eval helpers,
 * because those recurse through the module's evaluate and would leave
 * lambdas nested inside define, set!, and if forms unscanned.
 */
import { Effect, Option } from "effect";

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
  setVariableValue,
  symbol,
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
  RuntimeError,
  UnknownSyntax,
} from "../../packages/ch4/src/errors.js";
import { cons, type List, nil, toArray } from "../../packages/ch4/src/list.js";
import { format } from "../../packages/ch4/src/read.js";

const unassignedName = "*unassigned*";

const unassigned = (): Value => symbol(unassignedName);

const isUnassigned = (value: Value): boolean =>
  value._tag === "Symbol" && value.name === unassignedName;

/** One scanned-out internal definition. */
interface ScannedDefine {
  readonly name: SymbolValue;
  readonly value: Value;
}

interface ScannedBody {
  readonly defines: ReadonlyArray<ScannedDefine>;
  readonly rest: List<Value>;
}

const listOf = (items: ReadonlyArray<Value>): List<Value> =>
  items.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);

const list = (...items: Value[]): List<Value> => listOf(items);

const concatLists = (lists: ReadonlyArray<List<Value>>): List<Value> =>
  listOf(lists.flatMap((items) => toArray(items)));

/** Takes the leading defines of a body, as the text's scan does. */
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
 * The text's scan: the body's defines become one lambda application that
 * pre-binds every name to *unassigned* and then runs the set!s and the
 * remaining expressions.
 */
export const scanOutDefines = (body: List<Value>): List<Value> => {
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

/** A make-procedure whose bodies have their defines scanned out. */
export const makeProcedureScanned = (
  parameters: List<Value>,
  body: List<Value>,
  env: Env,
): CompoundProc => makeProcedure(parameters, scanOutDefines(body), env);

/**
 * The same-frame scan: the set!s and the remaining expressions, with no
 * wrapping let. The application pre-binds the names instead.
 */
export const scanOutDefinesSameFrame = (body: List<Value>): List<Value> => {
  const { defines, rest } = takeDefines(body);
  if (defines.length === 0) {
    return body;
  }
  return concatLists([listOf(defines.map((d) => list(symbol("set!"), d.name, d.value))), rest]);
};

const makeProcedureSameFrame = (
  parameters: List<Value>,
  body: List<Value>,
  env: Env,
): CompoundProc => makeProcedure(parameters, scanOutDefinesSameFrame(body), env);

/** The scanned-out names of a same-frame body, read off its set! forms. */
const scannedNames = (body: List<Value>): ReadonlyArray<SymbolValue> => {
  const names: SymbolValue[] = [];
  let rest: List<Value> = body;
  while (rest._tag === "Cons" && isAssignment(rest.head)) {
    names.push(assignmentVariable(rest.head));
    rest = rest.tail;
  }
  return names;
};

/**
 * The design answer: apply a scanned procedure with ONE new frame holding
 * the parameters plus the scanned names pre-bound to *unassigned*.
 */
const applyProcedureSameFrame = (
  procedure: Value,
  args: List<Value>,
  evalSequenceFrom: (exps: List<Value>, env: Env) => Effect.Effect<Value, EvaluationError>,
): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag !== "Compound") {
    return applyProcedure(procedure, args);
  }
  const names = scannedNames(procedure.body);
  return Effect.flatMap(
    extendEnvironment(
      concatLists([procedure.params, listOf(names)]),
      concatLists([args, listOf(names.map(() => unassigned()))]),
      procedure.env,
    ),
    (newEnv) => evalSequenceFrom(procedure.body, newEnv),
  );
};

/** Applies a compound body through this dispatch, not the module's. */
const applyProcedureFromDispatch = (
  procedure: Value,
  args: List<Value>,
  evalSequenceFrom: (exps: List<Value>, env: Env) => Effect.Effect<Value, EvaluationError>,
): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag !== "Compound") {
    return applyProcedure(procedure, args);
  }
  return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (newEnv) =>
    evalSequenceFrom(procedure.body, newEnv),
  );
};

/** Frames visible from an environment; env.vars and env.parent are public. */
export const frameDepth = (env: Env): number => {
  let depth = 1;
  let parent = env.parent;
  while (Option.isSome(parent)) {
    depth += 1;
    parent = parent.value.parent;
  }
  return depth;
};

/** The book's lookup plus the *unassigned* guard of the scanned scheme. */
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

/**
 * The full dispatch shared by the two scanned evaluators: same branch order
 * as the module's evaluate, with the lambda and application clauses swapped
 * and every recursion routed through this dispatch.
 */
const scannedDispatch = (sameFrame: boolean): Evaluate => {
  const makeProcedureVariant = sameFrame ? makeProcedureSameFrame : makeProcedureScanned;
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
      return Effect.succeed(makeProcedureVariant(lambdaParameters(exp), lambdaBody(exp), env));
    }
    if (isBegin(exp)) {
      return evalSequenceFrom(beginActions(exp), env);
    }
    if (isCond(exp)) {
      return evaluate(condToIf(exp), env);
    }
    if (isApplication(exp)) {
      return Effect.flatMap(evaluate(operator(exp), env), (procedure) =>
        Effect.flatMap(listOfValuesFrom(operands(exp), env), (args) =>
          sameFrame
            ? applyProcedureSameFrame(procedure, args, evalSequenceFrom)
            : applyProcedureFromDispatch(procedure, args, evalSequenceFrom),
        ),
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
  const evalSequenceFrom = (exps: List<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
    if (exps._tag === "Nil") {
      return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
    }
    if (isLastExp(exps)) {
      return evaluate(firstExp(exps), env);
    }
    return Effect.flatMap(evaluate(firstExp(exps), env), () =>
      evalSequenceFrom(restExps(exps), env),
    );
  };
  return evaluate;
};

/** Definitions interpreted with simultaneous scope, scanned out as a let. */
export const evaluateScanned: Evaluate = scannedDispatch(false);

/** Definitions interpreted with simultaneous scope in the SAME frame. */
export const evaluateSameFrame: Evaluate = scannedDispatch(true);

/** Reads a defined procedure value back out of an environment. */
export const lookupProcedure = (env: Env, name: string): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(lookupVariableValue(symbol(name), env), (value) =>
    value._tag === "Compound"
      ? Effect.succeed(value)
      : Effect.fail(
          new RuntimeError({ message: `not a procedure: ${name}`, detail: format(value) }),
        ),
  );

export function ex_4_17(): string {
  return (
    "Sequentially, e3 runs in the call frame holding u and v. Scanned out, the set!s run " +
    "inside the let's own frame, and the call frame sits behind it, so e3 sees two frames " +
    "where sequential interpretation shows one: the extra frame is the let's, a lambda " +
    "application whose only content is the *unassigned* pre-bindings. A correct program " +
    "can never tell the difference, because a scanned name is only read after its set! has " +
    "put a real value where the pre-binding was, and the chain still reaches every outer " +
    "binding. The interpreter can scan into the same frame: extend the call frame once " +
    "with the parameters plus the scanned names bound to *unassigned*, then run the " +
    "set!-only body; the frame counts of the two make-procedure variants differ by one, " +
    "and the same-frame variant computes the same answers."
  );
}
