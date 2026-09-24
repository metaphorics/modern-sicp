// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

/**
 * Environment operations over the frame chain of `core.Env`. Each frame is
 * a `Ref` holding a persistent `HashMap`: `define` and `set!` write through
 * the `Ref`, so every environment node sharing the frame sees the change —
 * the book's `define` and `set!` semantics, which a purely persistent chain
 * cannot express. `lookup` walks the chain and fails with `UnboundVariable`
 * when no frame binds the name.
 */
import { Effect, HashMap, Option, Ref } from "effect";

import type { Env, Value } from "./core.js";
import type { EvaluationError } from "./errors.js";
import { UnboundVariable } from "./errors.js";

/** A fresh global environment with no parent frame. */
export const makeGlobalEnv = (): Effect.Effect<Env> =>
  Effect.map(Ref.make(HashMap.empty<string, Value>()), (vars) => ({
    vars,
    parent: Option.none(),
  }));

/** A fresh empty frame extending `parent`, as a procedure call does. */
export const extendEnv = (parent: Env): Effect.Effect<Env> =>
  Effect.map(Ref.make(HashMap.empty<string, Value>()), (vars) => ({
    vars,
    parent: Option.some(parent),
  }));

/** Defines `name` in `env`'s own frame, shadowing outer frames. */
export const defineVariable = (env: Env, name: string, value: Value): Effect.Effect<void> =>
  Ref.update(env.vars, (vars) => HashMap.set(vars, name, value));

const findBinding = (env: Env, name: string): Effect.Effect<Option.Option<Value>> =>
  Effect.gen(function* () {
    const vars = yield* Ref.get(env.vars);
    const own = HashMap.get(vars, name);
    if (Option.isSome(own)) {
      return own;
    }
    if (Option.isNone(env.parent)) {
      return Option.none();
    }
    return yield* findBinding(env.parent.value, name);
  });

/** Looks `name` up along the frame chain, outermost last. */
export const lookupVariable = (env: Env, name: string): Effect.Effect<Value, EvaluationError> =>
  Effect.gen(function* () {
    const found = yield* findBinding(env, name);
    if (Option.isNone(found)) {
      return yield* Effect.fail(new UnboundVariable({ name }));
    }
    return found.value;
  });

/** Assigns `name` in the frame that defines it, like the book's `set!`:
 * the write lands in the shared frame, so every holder of the chain sees
 * it. An unbound name fails instead of defining it. */
export const setVariable = (
  env: Env,
  name: string,
  value: Value,
): Effect.Effect<void, EvaluationError> =>
  Effect.gen(function* () {
    const vars = yield* Ref.get(env.vars);
    if (Option.isSome(HashMap.get(vars, name))) {
      yield* Ref.update(env.vars, (v) => HashMap.set(v, name, value));
      return;
    }
    if (Option.isNone(env.parent)) {
      return yield* Effect.fail(new UnboundVariable({ name }));
    }
    yield* setVariable(env.parent.value, name, value);
  });
