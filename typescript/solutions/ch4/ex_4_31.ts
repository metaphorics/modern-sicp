// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.31: lazy and lazy-memo parameter declarations. The
 * declaration (name lazy) or (name lazy-memo) is object-language syntax
 * on the parameter list: a bare name is strict, the upward-compatible
 * default. The modes live beside the evaluator in a table keyed by the
 * compound procedure's identity, because the shared CompoundProc has no
 * slot for them, and the application clause binds each operand per its
 * mode through the tuning's delayOperand: strict evaluates now, lazy
 * delays into a recomputing wrapper, lazy-memo delays into a memoized
 * cell. Ordinary definitions register all-strict modes, so ordinary
 * programs keep Scheme's behavior.
 */
import { Effect } from "effect";

import {
  defineVariableValue,
  definitionValue,
  definitionVariable,
  isDefinition,
  isLambda,
  lambdaBody,
  lambdaParameters,
  makeProcedure,
  ok,
} from "../../packages/ch4/src/01-metacircular.js";
import {
  type LazyEvaluator,
  lazyDriverWith,
  makeLazyEvaluator,
} from "../../packages/ch4/src/02-lazy.js";
import type {
  CompoundProc,
  Env,
  Evaluate,
  SymbolValue,
  Value,
} from "../../packages/ch4/src/core.js";
import { type EvaluationError, UnknownSyntax } from "../../packages/ch4/src/errors.js";
import { type Cons, cons, type List, nil, toArray } from "../../packages/ch4/src/list.js";
import { format } from "../../packages/ch4/src/read.js";

/** The strictness a declared parameter asks for. */
export type ParamMode = "strict" | "lazy" | "lazy-memo";

/** Parses one parameter specification: a bare symbol is strict, and
 * (name lazy) or (name lazy-memo) declares its mode; anything else is
 * malformed and answers undefined. */
export const parseSpec = (
  spec: Value,
): { readonly name: SymbolValue; readonly mode: ParamMode } | undefined => {
  if (spec._tag === "Symbol") {
    return { name: spec, mode: "strict" };
  }
  if (spec._tag === "Cons") {
    const name = spec.head;
    const rest = spec.tail;
    const tag = rest._tag === "Cons" ? rest.head : undefined;
    const afterTag = rest._tag === "Cons" ? rest.tail : undefined;
    if (name._tag === "Symbol" && tag?._tag === "Symbol" && afterTag?._tag === "Nil") {
      if (tag.name === "lazy") {
        return { name, mode: "lazy" };
      }
      if (tag.name === "lazy-memo") {
        return { name, mode: "lazy-memo" };
      }
    }
  }
  return undefined;
};

/** Builds the declared evaluator: its mode table is private, its tuning
 * consults the table by procedure identity, and its lambda clause
 * registers the modes of each new declaration before the tuned dispatch
 * runs everything else. */
export const makeDeclaredEvaluator = (): LazyEvaluator => {
  const modes = new Map<CompoundProc, ReadonlyArray<ParamMode>>();
  const core = makeLazyEvaluator({
    delayOperand: (procedure: CompoundProc, position: number) => {
      const mode = modes.get(procedure)?.[position] ?? "strict";
      if (mode === "lazy") {
        return false;
      }
      return mode === "lazy-memo" ? true : undefined;
    },
  });
  /** Builds the procedure of one declared parameter list: the names
   * strip out for the frame, the modes register under identity. */
  const declaredLambda = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
    const known: Array<{ readonly name: SymbolValue; readonly mode: ParamMode }> = [];
    for (const spec of toArray(lambdaParameters(exp)).map(parseSpec)) {
      if (spec === undefined) {
        return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
      }
      known.push(spec);
    }
    const params: List<Value> = known.reduceRight<List<Value>>(
      (tail, spec) => cons(spec.name, tail),
      nil,
    );
    const procedure = makeProcedure(params, lambdaBody(exp), env);
    modes.set(
      procedure,
      known.map((spec) => spec.mode),
    );
    return Effect.succeed(procedure);
  };

  const evaluate: Evaluate = (exp, env) => {
    if (isLambda(exp)) {
      return declaredLambda(exp, env);
    }
    if (isDefinition(exp)) {
      const value = definitionValue(exp);
      if (isLambda(value)) {
        return Effect.flatMap(declaredLambda(value, env), (procedure) =>
          Effect.map(defineVariableValue(definitionVariable(exp), procedure, env), () => ok),
        );
      }
      return core.evaluate(exp, env);
    }
    return core.evaluate(exp, env);
  };
  return { ...core, evaluate };
};

/** The declared evaluator of the session. */
export const declaredEvaluator: LazyEvaluator = makeDeclaredEvaluator();

/** The book's session, with declared procedures and counting probes. */
export const session = [
  "(define count 0)",
  "(define (id n) (set! count (+ count 1)) n)",
  "(define (pick (x lazy)) 'taken)",
  "(pick (car '()))",
  "(define (f a (b lazy) c (d lazy-memo)) (list a b b c d d))",
  "(f (id 1) (id (+ 2 3)) (id 4) (id (* 5 6)))",
  "count",
  "(define (g (x lazy)) (list x x))",
  "(g (id 10))",
  "count",
  "(define (h (x lazy-memo)) (list x x))",
  "(h (id 10))",
  "count",
];

/** The printed values of the session. */
export const answers = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.map(lazyDriverWith(declaredEvaluator, session), (transcript) =>
    transcript.filter((_, i) => i % 4 === 3),
  );

export function ex_4_31(): string {
  const observed = Effect.runSync(answers());
  return (
    "The extension is upward-compatible: a bare parameter name is strict, " +
    "(name lazy) delays without memoization, and (name lazy-memo) delays " +
    "with it, so ordinary definitions like id behave as before. In the " +
    `session, (pick (car '())) answers ${observed[3]}: the lazy parameter ` +
    "is never demanded, so the (car '()) in it never runs. The declared f " +
    "binds a and c strictly at the call, b recomputes at each of its two " +
    "demands, and d computes once: (f (id 1) (id (+ 2 3)) (id 4) (id (* 5 " +
    `6))) answers ${observed[5]} with count ${observed[6]}. The same pair ` +
    "of disciplines alone: g's lazy parameter demanded twice runs twice, " +
    `count ${observed[9]}, and h's lazy-memo parameter demanded twice runs ` +
    `once more, count ${observed[12]}.`
  );
}
