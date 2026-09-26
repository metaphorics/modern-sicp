// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.11: frames as association lists. A frame is a list of
 * (name value) bindings held in a Ref, and the environment operations are
 * rewritten over that representation: lookup searches one frame's entries
 * before walking the chain, set! rewrites the first frame that binds the
 * name, and define conses a new binding onto the current frame. The
 * edition's pairs are proper lists, so a binding is the two-element list
 * (name value).
 */
import { Effect, Option, Ref } from "effect";

import { symbol } from "../../packages/ch4/src/01-metacircular.js";
import type { SymbolValue, Value } from "../../packages/ch4/src/core.js";
import { type EvaluationError, UnboundVariable } from "../../packages/ch4/src/errors.js";
import { cons, type List, list, nil } from "../../packages/ch4/src/list.js";

/** One frame: the book's list of (name value) bindings, in a Ref so that
 * define and set! can write it and every holder of the chain sees it. */
export type AssocFrame = Ref.Ref<List<Value>>;

/** The book's environment shape: a frame plus the enclosing environment. */
export interface AssocEnv {
  readonly frame: AssocFrame;
  readonly parent: Option.Option<AssocEnv>;
}

export const makeFrame = (): Effect.Effect<AssocFrame> => Ref.make<List<Value>>(nil);

export const makeGlobalAssocEnv = (): Effect.Effect<AssocEnv> =>
  Effect.map(makeFrame(), (frame) => ({ frame, parent: Option.none() }));

export const makeAssocEnv = (parent: AssocEnv): Effect.Effect<AssocEnv> =>
  Effect.map(makeFrame(), (frame) => ({ frame, parent: Option.some(parent) }));

const bindingName = (entry: Value): string | undefined =>
  entry._tag === "Cons" && entry.head._tag === "Symbol" ? entry.head.name : undefined;

const bindingValue = (entry: Value): Value =>
  entry._tag === "Cons" && entry.tail._tag === "Cons" ? entry.tail.head : nil;

const findInFrame = (bindings: List<Value>, name: string): Option.Option<Value> => {
  let rest = bindings;
  while (rest._tag === "Cons") {
    if (bindingName(rest.head) === name) {
      return Option.some(bindingValue(rest.head));
    }
    rest = rest.tail;
  }
  return Option.none();
};

/** The frame's names, newest binding first. */
export const frameVariables = (frame: AssocFrame): Effect.Effect<List<Value>> =>
  Effect.map(Ref.get(frame), (bindings) => {
    const names: Value[] = [];
    let rest = bindings;
    while (rest._tag === "Cons") {
      names.push(symbol(bindingName(rest.head) ?? "<malformed>"));
      rest = rest.tail;
    }
    return names.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);
  });

/** One more binding, consed onto this frame. */
export const addBindingToFrameAssoc = (
  frame: AssocFrame,
  variable: SymbolValue,
  value: Value,
): Effect.Effect<void> => Ref.update(frame, (bindings) => cons(list(variable, value), bindings));

export const lookupVariableValueAssoc = (
  variable: SymbolValue,
  env: AssocEnv,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(Ref.get(env.frame), (bindings) => {
    const own = findInFrame(bindings, variable.name);
    if (Option.isSome(own)) {
      return Effect.succeed(own.value);
    }
    const parent = env.parent;
    if (Option.isNone(parent)) {
      return Effect.fail(new UnboundVariable({ name: variable.name }));
    }
    return lookupVariableValueAssoc(variable, parent.value);
  });

const withReplacedBinding = (bindings: List<Value>, name: string, value: Value): List<Value> => {
  if (bindings._tag === "Nil") {
    return bindings;
  }
  if (bindingName(bindings.head) === name) {
    return cons(list(symbol(name), value), bindings.tail);
  }
  return cons(bindings.head, withReplacedBinding(bindings.tail, name, value));
};

export const setVariableValueAssoc = (
  variable: SymbolValue,
  value: Value,
  env: AssocEnv,
): Effect.Effect<void, EvaluationError> =>
  Effect.flatMap(Ref.get(env.frame), (bindings) => {
    if (Option.isSome(findInFrame(bindings, variable.name))) {
      return Ref.update(env.frame, (current) => withReplacedBinding(current, variable.name, value));
    }
    const parent = env.parent;
    if (Option.isNone(parent)) {
      return Effect.fail(new UnboundVariable({ name: variable.name }));
    }
    return setVariableValueAssoc(variable, value, parent.value);
  });

/** Defines in this frame, shadowing outer ones. */
export const defineVariableValueAssoc = (
  variable: SymbolValue,
  value: Value,
  env: AssocEnv,
): Effect.Effect<void> => addBindingToFrameAssoc(env.frame, variable, value);

export function ex_4_11(): string {
  return "A frame can be an association list instead of two parallel lists: one list of (name value) bindings, searched entry by entry. lookup-variable-value scans each frame's entries and then the chain; set-variable-value! rewrites the entries of the first frame that binds the name, so the write lands in the shared frame exactly as before; define-variable! conses the new binding onto the current frame. Behavior is unchanged, only the representation moved.";
}
