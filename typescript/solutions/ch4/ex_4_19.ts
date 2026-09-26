// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.19: Ben, Alyssa, and Eva debate the program in
 * `debatedProgram`. The exercise is answered empirically: one evaluator per
 * viewpoint, one shared dispatch. Sequential defines (the base rule) give
 * Ben's 16. Scanning out defines with the *unassigned* guard (Alyssa's
 * 4.16 mechanism) gives an error. Eva's rule, simultaneous definitions
 * whose value expressions see each other's final values, is implemented by
 * pre-binding every internal name to *unassigned* and forcing a name's
 * value expression on first read, memoizing it into the frame; the answer
 * is Eva's 20.
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
  listOfValues,
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
import { type Cons, cons, type List, nil, toArray } from "../../packages/ch4/src/list.js";
import { format, read } from "../../packages/ch4/src/read.js";

/** The program the three are arguing about. */
export const debatedProgram =
  "(let ((a 1)) (define (f x) (define b (+ a x)) (define a 5) (+ a b)) (f 10))";

/** Which internal-definition rule an evaluator follows. */
export type DefinitionScope = "sequential" | "scanned" | "eva";

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

/** Takes the leading defines of a body. */
const takeDefines = (
  body: List<Value>,
): { readonly defines: ReadonlyArray<ScannedDefine>; readonly rest: List<Value> } => {
  const defines: ScannedDefine[] = [];
  let rest: List<Value> = body;
  while (rest._tag === "Cons" && isDefinition(rest.head)) {
    defines.push({ name: definitionVariable(rest.head), value: definitionValue(rest.head) });
    rest = rest.tail;
  }
  return { defines, rest };
};

/** Alyssa's scanned body: one lambda application, *unassigned* bindings, set!s. */
const scanOutDefines = (body: List<Value>): List<Value> => {
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

const makeProcedureScanned = (parameters: List<Value>, body: List<Value>, env: Env): CompoundProc =>
  makeProcedure(parameters, scanOutDefines(body), env);

// ---------------------------------------------------------------------
// Eva's simultaneous rule: names forced on demand, memoized in the frame
// ---------------------------------------------------------------------

/** Per-call-frame state: each internal name's value expression and the set
 * of names currently being forced, for circularity detection. */
interface EvaFrame {
  readonly exprs: Map<string, Value>;
  readonly busy: Set<string>;
}

const evaFrames = new WeakMap<Env, EvaFrame>();

/** The defines stripped from each letrec-style procedure's body. */
const evaDefines = new WeakMap<CompoundProc, ReadonlyArray<ScannedDefine>>();

const makeProcedureEva = (parameters: List<Value>, body: List<Value>, env: Env): CompoundProc => {
  const { defines, rest } = takeDefines(body);
  const procedure = makeProcedure(parameters, rest, env);
  if (defines.length > 0) {
    evaDefines.set(procedure, defines);
  }
  return procedure;
};

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

/** Eva's lookup: forcing a *unassigned* name computes and memoizes its
 * value expression in this frame; a cycle is an error. */
const lookupEva = (
  variable: SymbolValue,
  env: Env,
  evaluate: Evaluate,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(lookupVariableValue(variable, env), (value) => {
    if (!isUnassigned(value)) {
      return Effect.succeed(value);
    }
    const frame = evaFrames.get(env);
    const expr = frame?.exprs.get(variable.name);
    if (frame === undefined || expr === undefined) {
      return Effect.fail(
        new RuntimeError({
          message: `variable used before assignment: ${variable.name}`,
          detail: unassignedName,
        }),
      );
    }
    if (frame.busy.has(variable.name)) {
      return Effect.fail(
        new RuntimeError({
          message: `circular internal definition: ${variable.name}`,
          detail: unassignedName,
        }),
      );
    }
    frame.busy.add(variable.name);
    return Effect.flatMap(
      Effect.flatMap(evaluate(expr, env), (final) =>
        Effect.map(setVariableValue(variable, final, env), () => final),
      ),
      (final) => {
        frame.busy.delete(variable.name);
        return Effect.succeed(final);
      },
    );
  });

const applyProcedureEva = (
  procedure: Value,
  args: List<Value>,
  evalSequenceFrom: (exps: List<Value>, env: Env) => Effect.Effect<Value, EvaluationError>,
): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag !== "Compound") {
    return applyProcedure(procedure, args);
  }
  const defines = evaDefines.get(procedure) ?? [];
  return Effect.flatMap(
    extendEnvironment(
      concatLists([procedure.params, listOf(defines.map((d) => d.name))]),
      concatLists([args, listOf(defines.map(() => unassigned()))]),
      procedure.env,
    ),
    (newEnv) => {
      evaFrames.set(newEnv, {
        exprs: new Map(defines.map((d) => [d.name.name, d.value])),
        busy: new Set<string>(),
      });
      return evalSequenceFrom(procedure.body, newEnv);
    },
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

// ---------------------------------------------------------------------
// let, so the debated program can be written as the book writes it
// ---------------------------------------------------------------------

export const isLet = (exp: Value): exp is Cons<Value> =>
  exp._tag === "Cons" && exp.head._tag === "Symbol" && exp.head.name === "let";

/** The book's let->combination: ((lambda (names...) body...) inits...). */
export const letToCombination = (exp: Cons<Value>): Value => {
  const bindingList = exp.tail;
  if (bindingList._tag !== "Cons") {
    throw new RuntimeError({ message: "malformed let: missing bindings", detail: format(exp) });
  }
  const names: Value[] = [];
  const inits: Value[] = [];
  const first = bindingList.head;
  let rest: List<Value>;
  if (first._tag === "Cons" || first._tag === "Nil") {
    rest = first;
  } else {
    throw new RuntimeError({
      message: "malformed let: bindings must be a list",
      detail: format(first),
    });
  }
  while (rest._tag === "Cons") {
    const binding = rest.head;
    if (binding._tag !== "Cons" || binding.tail._tag !== "Cons") {
      throw new RuntimeError({ message: "malformed let binding", detail: format(binding) });
    }
    names.push(binding.head);
    inits.push(binding.tail.head);
    rest = rest.tail;
  }
  const lambda = cons<Value>(symbol("lambda"), cons(listOf(names), bindingList.tail));
  return cons<Value>(lambda, listOf(inits));
};

// ---------------------------------------------------------------------
// the three evaluators over one dispatch
// ---------------------------------------------------------------------

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

/** Builds the evaluator that follows `scope`'s rule for internal defines. */
export const makeDefineScopingEvaluator = (scope: DefinitionScope): Evaluate => {
  const evaluate: Evaluate = (exp, env) => {
    if (isSelfEvaluating(exp)) {
      return Effect.succeed(exp);
    }
    if (isVariable(exp)) {
      if (scope === "sequential") {
        return lookupVariableValue(exp, env);
      }
      if (scope === "scanned") {
        return lookupChecked(exp, env);
      }
      return lookupEva(exp, env, evaluate);
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
    if (isLet(exp)) {
      return evaluate(letToCombination(exp), env);
    }
    if (isLambda(exp)) {
      const maker =
        scope === "sequential"
          ? makeProcedure
          : scope === "scanned"
            ? makeProcedureScanned
            : makeProcedureEva;
      return Effect.succeed(maker(lambdaParameters(exp), lambdaBody(exp), env));
    }
    if (isBegin(exp)) {
      return evalSequenceWith(evaluate, beginActions(exp), env);
    }
    if (isCond(exp)) {
      return evaluate(condToIf(exp), env);
    }
    if (isApplication(exp)) {
      return Effect.flatMap(evaluate(operator(exp), env), (procedure) =>
        Effect.flatMap(
          scope === "sequential"
            ? listOfValues(operands(exp), env)
            : listOfValuesFrom(operands(exp), env),
          (args) => {
            if (scope === "sequential") {
              return applyProcedure(procedure, args);
            }
            if (scope === "scanned") {
              return applyProcedureFromDispatch(procedure, args, (exps2, env2) =>
                evalSequenceWith(evaluate, exps2, env2),
              );
            }
            return applyProcedureEva(procedure, args, (exps2, env2) =>
              evalSequenceWith(evaluate, exps2, env2),
            );
          },
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
  return evaluate;
};

/** Runs the debated program under one of the three rules. */
export const runDebatedProgram = (scope: DefinitionScope): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    makeDefineScopingEvaluator(scope)(read(debatedProgram), env),
  );

export function ex_4_19(): string {
  return (
    "The sequential rule gives 16, Alyssa's scanned simultaneous rule gives an error, and " +
    "Eva's simultaneous rule with value expressions that see each other's final values " +
    "gives 20. I support the Alyssa/MIT position for real systems: when a program's " +
    "internal defines violate the restrictions that make the readings agree, an error is " +
    "better than whichever answer falls out of the mechanism. Eva's reading is still " +
    "implementable, and this file implements it: pre-bind every internal name to " +
    "*unassigned* and force each name's value expression on first read, memoizing the " +
    "value into the frame, so every expression sees final values; a forced name that " +
    "reads itself is a circular definition and errors."
  );
}
