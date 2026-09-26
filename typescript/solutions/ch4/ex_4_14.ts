// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.14: two ways to give the evaluator a map. Louis installs the
 * host's Array.prototype.map as a primitive: like the book's
 * apply-primitive-procedure, it hands the host the procedure's underlying
 * implementation, so it reaches a primitive's host function but throws a
 * host TypeError when handed an evaluator closure object; the failure is
 * surfaced as a RuntimeError. Eva Lu Ator types the definition of map
 * into the object language itself, where every call goes through the
 * evaluator's apply, so both calls work.
 */
import { Effect } from "effect";

import {
  addBindingToFrame,
  evalString,
  setupEnvironment,
  symbol,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Primitive, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { RuntimeError } from "../../packages/ch4/src/errors.js";
import { cons, type List, list, nil, toArray } from "../../packages/ch4/src/list.js";

const asList = (value: Value): List<Value> =>
  value._tag === "Cons" || value._tag === "Nil" ? value : nil;

/** Louis's map: the host Array.prototype.map, wrapped as a primitive. */
const louisMap: Primitive = (args) => {
  const [fnValue, listValue] = toArray(args);
  const items = toArray(asList(listValue ?? nil));
  return Effect.try({
    try: () => {
      const fn = fnValue ?? nil;
      // The implementation a primitive offers the host. Anything else is
      // handed over as-is: the host call of the object itself is the
      // TypeError the book predicts, caught just below.
      const candidate: unknown = fn;
      const implementation =
        fn._tag === "Primitive"
          ? fn.fn
          : (candidate as (args: List<Value>) => Effect.Effect<Value, EvaluationError>);
      const mapped = items.map((item) => Effect.runSync(implementation(list(item))));
      return mapped.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);
    },
    catch: (error) =>
      new RuntimeError({
        message: "map: the host could not call this procedure",
        detail: error instanceof Error ? error.message : String(error),
      }),
  });
};

/** The global environment with Louis's map installed as a primitive. */
export const makeLouisEnvironment = (): Effect.Effect<Env, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    Effect.map(
      addBindingToFrame(symbol("map"), { _tag: "Primitive", name: "map", fn: louisMap }, env),
      () => env,
    ),
  );

/** Eva's map: the definition typed into the object language. */
export const evaMapDefinition =
  "(define (map p x) (if (null? x) '() (cons (p (car x)) (map p (cdr x)))))";

/** The global environment with Eva's object-language map defined. */
export const makeEvaEnvironment = (): Effect.Effect<Env, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    Effect.map(evalString(evaMapDefinition, env), () => env),
  );

export function ex_4_14(): string {
  return "Louis's installed host map works only when the mapped procedure can be called by the host: (map car '((1 2) (3 4))) answers (1 3), but mapping a lambda hands Array.prototype.map an evaluator closure object, which is not a host function, and the call dies with a TypeError surfaced as a RuntimeError. Eva types map's definition into the object language, where the per-element call goes through the evaluator's own apply, so both calls work: her map is just another compound procedure.";
}
