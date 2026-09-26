// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.12: one walk, three procedures. forEachBinding visits every
 * binding of one frame; walkEnvChain visits the frames of a chain until a
 * probe finds what it is looking for. lookup-variable-value and
 * set-variable-value! are then two probes over the same walk, and
 * define-variable! needs no traversal at all: it always writes the first
 * frame, which is the book's own answer for define.
 */
import { Effect, HashMap, Option, Ref } from "effect";

import type { Env, SymbolValue, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { UnboundVariable } from "../../packages/ch4/src/errors.js";

/** Runs visit over every binding of one frame, in no promised order. */
export const forEachBinding = (
  env: Env,
  visit: (name: string, value: Value) => Effect.Effect<void>,
): Effect.Effect<void, EvaluationError> =>
  Effect.flatMap(Ref.get(env.vars), (vars) =>
    Effect.map(
      Effect.forEach(Array.from(HashMap.entries(vars)), ([name, value]) => visit(name, value)),
      () => undefined,
    ),
  );

/** Walks the chain frame by frame; the first frame whose probe answers
 * with a value ends the walk, an exhausted chain answers with nothing. */
export const walkEnvChain = <A>(
  env: Env,
  visitFrame: (frame: Env) => Effect.Effect<Option.Option<A>>,
): Effect.Effect<Option.Option<A>> => {
  const parent = env.parent;
  if (Option.isNone(parent)) {
    return visitFrame(env);
  }
  return Effect.flatMap(visitFrame(env), (found) =>
    Option.isSome(found) ? Effect.succeed(found) : walkEnvChain(parent.value, visitFrame),
  );
};

const findInFrame = (frame: Env, name: string): Effect.Effect<Option.Option<Value>> =>
  Effect.map(Ref.get(frame.vars), (vars) => HashMap.get(vars, name));

const frameWithBinding = (frame: Env, name: string): Effect.Effect<Option.Option<Env>> =>
  Effect.map(Ref.get(frame.vars), (vars) =>
    HashMap.has(vars, name) ? Option.some(frame) : Option.none(),
  );

export const lookupVariableValueTraversing = (
  variable: SymbolValue,
  env: Env,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(
    walkEnvChain(env, (frame) => findInFrame(frame, variable.name)),
    (found) =>
      Option.isSome(found)
        ? Effect.succeed(found.value)
        : Effect.fail(new UnboundVariable({ name: variable.name })),
  );

export const setVariableValueTraversing = (
  variable: SymbolValue,
  value: Value,
  env: Env,
): Effect.Effect<void, EvaluationError> =>
  Effect.flatMap(
    walkEnvChain(env, (frame) => frameWithBinding(frame, variable.name)),
    (found) =>
      Option.isSome(found)
        ? Ref.update(found.value.vars, (vars) => HashMap.set(vars, variable.name, value))
        : Effect.fail(new UnboundVariable({ name: variable.name })),
  );

/** Define writes the first frame and needs no walk. */
export const defineVariableValueTraversing = (
  variable: SymbolValue,
  value: Value,
  env: Env,
): Effect.Effect<void> => Ref.update(env.vars, (vars) => HashMap.set(vars, variable.name, value));

export function ex_4_12(): string {
  return "The three environment procedures share one shape: walk the chain looking for a name. Factored out, the walk is two abstract operations, forEachBinding over one frame's bindings and walkEnvChain over the frames, and lookup and set! collapse into probes over that walk. define-variable! needs no traversal: it always writes the first frame, so it stays a direct frame write.";
}
