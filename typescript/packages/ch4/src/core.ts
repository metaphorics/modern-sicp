// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

/**
 * The evaluator's core data: values, environments, and the shape of
 * `evaluate`. The book's evaluator works on list-structured expressions,
 * so an expression is itself just a `Value`: a symbol, a cons pair, or a
 * leaf. There is no separately parsed form; the syntax predicates of the
 * metacircular evaluator look at the data with the same car/cdr surgery
 * the book uses. The cons pairs come from `list.ts`, the mutable frame
 * chain from `env.ts`, and the checked failures from `errors.ts`.
 */
import type { Effect, HashMap, Option, Ref } from "effect";

import type { EvaluationError } from "./errors.js";
import type { Cons, List, Nil } from "./list.js";

/** A number leaf of the object language. */
export interface NumberValue {
  readonly _tag: "Number";
  readonly n: number;
}

/** A boolean leaf; the book's true and false objects. */
export interface BooleanValue {
  readonly _tag: "Boolean";
  readonly b: boolean;
}

/** A string leaf, the book's `"text"` datum. */
export interface StringValue {
  readonly _tag: "String";
  readonly s: string;
}

/** A symbol: variable names, special-form tags, and quoted names share it. */
export interface SymbolValue {
  readonly _tag: "Symbol";
  readonly name: string;
}

/** The value of `display` and `newline`: nothing worth printing. */
export interface UnspecifiedValue {
  readonly _tag: "Unspecified";
}

/** The book's empty list, `nil`. */
export type EmptyList = Nil;

/** A cons pair whose two halves are evaluator values. */
export type Pair = Cons<Value>;

/** A primitive procedure: a name plus a host function over evaluated arguments. */
export interface PrimitiveValue {
  readonly _tag: "Primitive";
  readonly name: string;
  readonly fn: Primitive;
}

/** A compound procedure: parameter symbols, a body of expressions, and the
 * defining environment (section 4.1.3). */
export interface CompoundProc {
  readonly _tag: "Compound";
  readonly params: List<Value>;
  readonly body: List<Value>;
  readonly env: Env;
}

/**
 * An execution procedure (section 4.1.7) held in the body slot of a
 * compound procedure. The analyzed evaluator stores host closures where
 * the direct evaluator stores expression lists; this wrapper gives the
 * closure a place in the `Value` union without a second procedure record.
 */
export interface ExecutionValue {
  readonly _tag: "Execution";
  readonly run: (env: Env) => Effect.Effect<Value, EvaluationError>;
}

/** Everything the evaluator reads or produces. */
export type Value =
  | NumberValue
  | BooleanValue
  | StringValue
  | SymbolValue
  | UnspecifiedValue
  | EmptyList
  | Pair
  | PrimitiveValue
  | CompoundProc
  | ExecutionValue;

/** A primitive: applied to the evaluated argument list, it yields one value. */
export type Primitive = (args: List<Value>) => Effect.Effect<Value, EvaluationError>;

/**
 * The environment: one `Ref`-held `HashMap` frame plus the enclosing
 * environment; the global environment is the frame with no parent. The
 * `Ref` is what makes `set!` real: every environment value sharing the
 * frame observes the write, as section 4.1.3 requires.
 */
export interface Env {
  readonly vars: Ref.Ref<HashMap.HashMap<string, Value>>;
  readonly parent: Option.Option<Env>;
}

/** `(exp, env) => Effect<Value, EvaluationError>` — the book's `eval`. */
export type Evaluate = (exp: Value, env: Env) => Effect.Effect<Value, EvaluationError>;

/** Section 4.1.7's `analyze`: compile once, then apply to many environments. */
export type Analyze = (exp: Value) => (env: Env) => Effect.Effect<Value, EvaluationError>;
