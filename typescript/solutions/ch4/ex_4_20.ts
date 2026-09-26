// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.20: letrec as a derived expression. A letrec whose bindings are
 * (v1 e1) ... (vn en) and whose body is b transforms into a let that
 * pre-binds every v to '*unassigned*', followed by one set! per binding and
 * then b. Because the derived code still contains let and set!, the
 * dispatch here is complete: it handles letrec, the derived let, and every
 * other form, so letrec works nested inside lambda bodies. The tests also
 * pin what is loose about Louis's claim that a plain let can replace
 * letrec: let's inits are evaluated outside the new bindings, so a binding
 * whose init reads a sibling fails with let and works with letrec.
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
import { type Cons, cons, type List, nil, toArray } from "../../packages/ch4/src/list.js";
import { format, read } from "../../packages/ch4/src/read.js";

const unassignedName = "*unassigned*";

const unassigned = (): SymbolValue => symbol(unassignedName);

const listOf = (items: ReadonlyArray<Value>): List<Value> =>
  items.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);

const list = (...items: Value[]): List<Value> => listOf(items);

const concatLists = (lists: ReadonlyArray<List<Value>>): List<Value> =>
  listOf(lists.flatMap((items) => toArray(items)));

const tagged = (tag: string, exp: Value): exp is Cons<Value> =>
  exp._tag === "Cons" && exp.head._tag === "Symbol" && exp.head.name === tag;

export const isLetrec = (exp: Value): exp is Cons<Value> => tagged("letrec", exp);

export const isLet = (exp: Value): exp is Cons<Value> => tagged("let", exp);

interface Bindings {
  readonly names: Value[];
  readonly inits: Value[];
  readonly body: List<Value>;
}

/** Reads a let/letrec binding list into parallel name and init arrays. */
const readBindings = (exp: Cons<Value>): Bindings => {
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
      throw new RuntimeError({ message: "malformed binding", detail: format(binding) });
    }
    names.push(binding.head);
    inits.push(binding.tail.head);
    rest = rest.tail;
  }
  return { names, inits, body: bindingList.tail };
};

/** The book's let->combination: ((lambda (names...) body...) inits...). */
export const letToCombination = (exp: Cons<Value>): Value => {
  const { names, inits, body } = readBindings(exp);
  const lambda = cons<Value>(symbol("lambda"), cons(listOf(names), body));
  return cons<Value>(lambda, listOf(inits));
};

/**
 * letrec->let: (let ((v1 '*unassigned*) ...) (set! v1 e1) ... body...).
 * The shape is exactly the book's; the derived let is itself derived again
 * by the dispatch's let clause.
 */
export const letrecToLet = (exp: Cons<Value>): Value => {
  const { names, inits, body } = readBindings(exp);
  const preBindings = listOf(names.map((name) => list(name, list(symbol("quote"), unassigned()))));
  const setForms: Value[] = [];
  names.forEach((name, i) => {
    const init = inits[i];
    if (init === undefined) {
      throw new RuntimeError({ message: "binding without init", detail: format(name) });
    }
    setForms.push(list(symbol("set!"), name, init));
  });
  return cons<Value>(symbol("let"), cons(preBindings, concatLists([listOf(setForms), body])));
};

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

/** The full dispatch with letrec (and its derived let) added. */
export const evaluateWithLetrec: Evaluate = (() => {
  const evaluate: Evaluate = (exp, env) => {
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
    if (isLetrec(exp)) {
      return evaluate(letrecToLet(exp), env);
    }
    if (isLet(exp)) {
      return evaluate(letToCombination(exp), env);
    }
    if (isLambda(exp)) {
      return Effect.succeed(makeProcedure(lambdaParameters(exp), lambdaBody(exp), env));
    }
    if (isBegin(exp)) {
      return evalSequenceWith(evaluate, beginActions(exp), env);
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
            (newEnv) => evalSequenceWith(evaluate, procedure.body, newEnv),
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
  return evaluate;
})();

/** Evaluates source in a fresh global environment with letrec support. */
export const runWithLetrec = (source: string): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) => evaluateWithLetrec(read(source), env));

/** Evaluates forms in order in ONE global environment, answering the last value. */
export const runProgramsWithLetrec = (
  sources: ReadonlyArray<string>,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    Effect.flatMap(
      Effect.forEach(sources, (source) => evaluateWithLetrec(read(source), env)),
      (values) => {
        const last = values[values.length - 1];
        return last !== undefined
          ? Effect.succeed(last)
          : Effect.die(new Error("no forms evaluated"));
      },
    ),
  );

export function ex_4_20(): string {
  return (
    "letrec transforms into a let that pre-binds every name to *unassigned* and then " +
    "assigns each with set!, so the bindings exist simultaneously and the value " +
    "expressions can be mutually recursive. What is loose about Louis's reasoning: a " +
    "plain let evaluates its inits outside the new bindings, so it happens to work for " +
    "lambda inits, which defer every read until a call, but fails as soon as an init " +
    "reads a sibling binding; letrec's let-and-set! shape is what gives the names their " +
    "simultaneous scope."
  );
}
