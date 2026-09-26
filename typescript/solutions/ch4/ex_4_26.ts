// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.26: the unless debate. Ben's side implements unless as a
 * derived expression on the strict 4.1 evaluator: (unless c u e) rewrites
 * into (if c e u) before evaluation, so it works over armed calls and
 * composes with the object language's map, but the name stays syntax, and
 * `(list unless)` fails with an unbound variable. Alyssa's side keeps
 * unless an ordinary procedure under the lazy evaluator: the arms delay,
 * the same armed call and mapping answer, and the procedure stays
 * first-class, so it can be stored in a list and applied from operator
 * position, which no special form can be.
 */
import { Effect } from "effect";
import {
  evaluate,
  isSymbol,
  makeIf,
  setupEnvironment,
  taggedList,
} from "../../packages/ch4/src/01-metacircular.js";
import { lazyDriverWith, lazyEvaluator } from "../../packages/ch4/src/02-lazy.js";
import type { Evaluate, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError, UnboundVariable } from "../../packages/ch4/src/errors.js";
import type { Cons, List } from "../../packages/ch4/src/list.js";
import { cons } from "../../packages/ch4/src/list.js";
import { format, read } from "../../packages/ch4/src/read.js";

export const unlessDefinition =
  "(define (unless condition usual-value exceptional-value) (if condition exceptional-value usual-value))";

export const mapDefinition =
  "(define (map f items) (if (null? items) (list) (cons (f (car items)) (map f (cdr items)))))";

export const armedCall = "(unless (= 1 1) (/ 1 0) 42)";

export const mappedCall = "(map (lambda (b) (unless b 0 7)) (list #f #t))";

/** The book's unless? */
export const isUnless = (exp: Value): exp is Cons<Value> => taggedList("unless", exp);

/** The derived expression: (unless c u e) becomes (if c e u). */
export const unlessToIf = (exp: Value): Value => {
  if (!isUnless(exp)) {
    return exp;
  }
  const parts: Value[] = [];
  let rest: List<Value> = exp.tail;
  while (rest._tag === "Cons" && parts.length < 3) {
    parts.push(rest.head);
    rest = rest.tail;
  }
  const condition = parts[0];
  const usual = parts[1];
  const exceptional = parts[2];
  if (condition === undefined || usual === undefined || exceptional === undefined) {
    return exp;
  }
  return makeIf(condition, exceptional, usual);
};

const expandUnlessList = (seq: List<Value>): List<Value> =>
  seq._tag === "Cons" ? cons(expandUnless(seq.head), expandUnlessList(seq.tail)) : seq;

/** Ben's rewrite, over the whole program tree: every unless call becomes
 * an if before evaluation starts, because the strict evaluator's bodies
 * run through its own dispatch and would never see a per-form hook.
 * Quoted data is left untouched. */
export function expandUnless(exp: Value): Value {
  if (exp._tag !== "Cons") {
    return exp;
  }
  // Quoted data is left untouched: (quote (a unless b)) is data.
  if (isSymbol(exp.head) && exp.head.name === "quote") {
    return exp;
  }
  if (isSymbol(exp.head) && exp.head.name === "unless") {
    return unlessToIf(exp);
  }
  return cons(expandUnless(exp.head), expandUnlessList(exp.tail));
}

/** Ben's evaluator: the strict dispatch running unless-free programs. */
export const evaluateWithUnless: Evaluate = (exp, env) => evaluate(expandUnless(exp), env);

const strictRun = (
  sources: ReadonlyArray<string>,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    Effect.forEach(sources, (source) =>
      Effect.map(evaluateWithUnless(read(source), env), (value) => format(value)),
    ),
  );

const unboundOf = (
  effect: Effect.Effect<unknown, EvaluationError>,
): Effect.Effect<UnboundVariable, EvaluationError> =>
  Effect.flatMap(Effect.result(effect), (outcome) => {
    if (outcome._tag === "Failure" && outcome.failure._tag === "UnboundVariable") {
      return Effect.succeed(outcome.failure);
    }
    return Effect.die(new Error("expected an UnboundVariable"));
  });

/** Ben's session: the armed call, the mapping, and the name as a value. */
export const benSession = (): Effect.Effect<
  { readonly armed: string; readonly mapped: string; readonly nameError: string },
  EvaluationError
> =>
  Effect.flatMap(strictRun([armedCall]), (armed) =>
    Effect.flatMap(strictRun([mapDefinition, mappedCall]), (mapped) =>
      Effect.map(unboundOf(strictRun(["(list unless)"])), (error) => ({
        armed: armed[0] ?? "",
        mapped: mapped[mapped.length - 1] ?? "",
        nameError: `${error._tag}: ${error.name}`,
      })),
    ),
  );

/** Alyssa's session: unless stays a procedure under the lazy evaluator. */
export const alyssaSession = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  lazyDriverWith(lazyEvaluator, [
    unlessDefinition,
    mapDefinition,
    armedCall,
    mappedCall,
    "(define choices (list unless))",
    "((car choices) #f 7 (/ 1 0))",
  ]);

/** The observed answers of both sides, in order. */
export const answers = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(benSession(), (ben) =>
    Effect.map(alyssaSession(), (alyssa) => [
      ben.armed,
      ben.mapped,
      ben.nameError,
      ...alyssa.filter((_, i) => i % 4 === 3).slice(2),
    ]),
  );

export function ex_4_26(): string {
  const observed = Effect.runSync(answers());
  return (
    "Ben is right that a special form can do the work: with unless derived " +
    `into an if, the armed call answers ${observed[0]} and the mapping over ` +
    `(list #f #t) answers ${observed[1]}, because the rewrite picks the arm ` +
    "before anything evaluates. But the name is merely syntax: evaluating " +
    "unless as a variable fails with " +
    `"${observed[2]}", so unless cannot be passed to or returned from ` +
    "higher-order procedures. Alyssa's procedure version under the lazy " +
    `evaluator answers ${observed[3]} for the armed call and ${observed[4]} ` +
    "for the mapping, and the procedure stays first-class: storing it with " +
    "(list unless) and applying the retrieved value answers " +
    `${observed[5]}, an armed use that is impossible for a special form.`
  );
}
