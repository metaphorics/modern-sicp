// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.9: iteration constructs as derived expressions. The language
 * gains `while` and `for`: each expands, at evaluation time, into a core
 * language combination that installs a local recursive loop procedure in a
 * freshly named frame slot and calls it. The dispatch below is complete,
 * so the constructs work anywhere an expression can appear, including
 * nested lambda bodies, and no make-procedure or host-apply trick is used.
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
  makeBegin,
  makeIf,
  makeLambda,
  makeProcedure,
  ok,
  operands,
  operator,
  setVariableValue,
  symbol,
  taggedList,
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
  NotAProcedure,
  RuntimeError,
  UnknownSyntax,
} from "../../packages/ch4/src/errors.js";
import { type Cons, cons, type List, list, nil, toArray } from "../../packages/ch4/src/list.js";
import { format, ReadError, read } from "../../packages/ch4/src/read.js";

const cadr = (exp: Cons<Value>): Value =>
  exp.tail._tag === "Cons" ? exp.tail.head : symbol("<malformed>");

const cddr = (exp: Cons<Value>): List<Value> => (exp.tail._tag === "Cons" ? exp.tail.tail : nil);

const fromValues = (items: ReadonlyArray<Value>): List<Value> =>
  items.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);

const QUOTED_NIL: Value = list<Value>(symbol("quote"), nil);

// The loop constructs ---------------------------------------------------

export const isWhile = (exp: Value): exp is Cons<Value> => taggedList("while", exp);

export const whilePredicate = (exp: Cons<Value>): Value => cadr(exp);

export const whileActions = (exp: Cons<Value>): List<Value> => cddr(exp);

/** (while pred body...) becomes a combination that binds a fresh loop
 * cell, installs in it a procedure that re-tests pred and, when true,
 * runs body and recurses, then calls the loop. Its value is the empty
 * list. */
export const whileToCombination = (exp: Cons<Value>): Value => {
  const loop = freshCell("while-loop");
  const again = list<Value>(loop);
  const step = fromValues([...toArray(whileActions(exp)), again]);
  const loopBody = makeIf(whilePredicate(exp), makeBegin(step), QUOTED_NIL);
  const install = list<Value>(symbol("set!"), loop, makeLambda(nil, list<Value>(loopBody)));
  const kickoff = makeBegin(list<Value>(install, again));
  return list<Value>(makeLambda(list<Value>(loop), list<Value>(kickoff)), QUOTED_NIL);
};

export interface ForBindingSpec {
  readonly variable: SymbolValue;
  readonly from: Value;
  readonly to: Value;
}

const bindingParts = (binding: Value): ForBindingSpec | undefined => {
  if (binding._tag !== "Cons" || binding.head._tag !== "Symbol") {
    return undefined;
  }
  if (binding.tail._tag !== "Cons" || binding.tail.tail._tag !== "Cons") {
    return undefined;
  }
  return { variable: binding.head, from: binding.tail.head, to: binding.tail.tail.head };
};

export const isFor = (exp: Value): exp is Cons<Value> => {
  if (!taggedList("for", exp)) {
    return false;
  }
  return bindingParts(cadr(exp)) !== undefined;
};

/** (for (v from to) body...) binds to once, installs a loop procedure
 * over v, and walks v from from up to and including to. Its value is the
 * empty list. */
export const forToCombination = (exp: Cons<Value>): Value => {
  const parts = bindingParts(cadr(exp));
  if (parts === undefined) {
    return list<Value>(exp); // unreachable through isFor; kept total for the type
  }
  const { variable, from, to } = parts;
  const limit = freshCell("for-limit");
  const loop = freshCell("for-loop");
  const one: Value = { _tag: "Number", n: 1 };
  const again = list<Value>(loop, list<Value>(symbol("+"), variable, one));
  const step = fromValues([...toArray(cddr(exp)), again]);
  const loopBody = makeIf(
    list<Value>(symbol("<"), variable, list<Value>(symbol("+"), limit, one)),
    makeBegin(step),
    QUOTED_NIL,
  );
  const install = list<Value>(
    symbol("set!"),
    loop,
    makeLambda(list<Value>(variable), list<Value>(loopBody)),
  );
  const kickoff = list<Value>(loop, from);
  const inner = makeBegin(list<Value>(install, kickoff));
  const loopApplication = list<Value>(
    makeLambda(list<Value>(loop), list<Value>(inner)),
    QUOTED_NIL,
  );
  return list<Value>(makeLambda(list<Value>(limit), list<Value>(loopApplication)), to);
};

let expansionCount = 0;

const freshCell = (base: string): SymbolValue => {
  expansionCount += 1;
  return symbol(`${base}:${expansionCount}`);
};

// A complete dispatch: the module's evaluate with the two loop clauses
// added, so while and for work in nested bodies too.

const evalIfLoops = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateWithLoops(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate)
      ? evaluateWithLoops(ifConsequent(exp), env)
      : evaluateWithLoops(ifAlternative(exp), env),
  );

const evalSequenceLoops = (exps: List<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
  if (exps._tag === "Nil") {
    return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
  }
  if (exps.tail._tag === "Nil") {
    return evaluateWithLoops(exps.head, env);
  }
  return Effect.flatMap(evaluateWithLoops(exps.head, env), () => evalSequenceLoops(exps.tail, env));
};

const evalAssignmentLoops = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateWithLoops(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

const evalDefinitionLoops = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateWithLoops(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

const listOfValuesLoops = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> => {
  if (exps._tag === "Nil") {
    return Effect.succeed(nil);
  }
  return Effect.flatMap(evaluateWithLoops(exps.head, env), (first) =>
    Effect.map(listOfValuesLoops(exps.tail, env), (rest) => cons(first, rest)),
  );
};

const applyLoops = (procedure: Value, args: List<Value>): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag === "Primitive") {
    return applyPrimitiveProcedure(procedure, args);
  }
  if (procedure._tag === "Compound") {
    return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (callEnv) =>
      evalSequenceLoops(procedure.body, callEnv),
    );
  }
  return Effect.fail(new NotAProcedure({ value: format(procedure) }));
};

const evalApplicationLoops = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluateWithLoops(operator(exp), env), (procedure) =>
    Effect.flatMap(listOfValuesLoops(operands(exp), env), (args) => applyLoops(procedure, args)),
  );

const evalWhile = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  evaluateWithLoops(whileToCombination(exp), env);

const evalFor = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  evaluateWithLoops(forToCombination(exp), env);

export const evaluateWithLoops: Evaluate = (exp, env) => {
  if (isWhile(exp)) {
    return evalWhile(exp, env);
  }
  if (isFor(exp)) {
    return evalFor(exp, env);
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
    return evalAssignmentLoops(exp, env);
  }
  if (isDefinition(exp)) {
    return evalDefinitionLoops(exp, env);
  }
  if (isIf(exp)) {
    return evalIfLoops(exp, env);
  }
  if (isLambda(exp)) {
    const procedure: CompoundProc = makeProcedure(lambdaParameters(exp), lambdaBody(exp), env);
    return Effect.succeed(procedure);
  }
  if (isBegin(exp)) {
    return evalSequenceLoops(beginActions(exp), env);
  }
  if (isCond(exp)) {
    return evaluateWithLoops(condToIf(exp), env);
  }
  if (isApplication(exp)) {
    return evalApplicationLoops(exp, env);
  }
  return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
};

/** Reads one form and evaluates it with the loop constructs installed. */
export const evalStringWithLoops = (
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
    (exp) => evaluateWithLoops(exp, env),
  );

export function ex_4_09(): string {
  return "Iteration belongs in the evaluator, not in the programmer's discipline: while and for are derived expressions that expand at evaluation time into a core-language combination holding a local recursive loop procedure in a freshly named frame slot. The expansion terminates by the ordinary application rule, uses no make-procedure or host-apply escape hatch, and works anywhere an expression can appear, including nested lambda bodies. A for loop summing 1 through 5 answers 15, exactly what manual recursion computes for (sum-to 5).";
}
