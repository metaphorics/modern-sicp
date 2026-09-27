// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 and 4.2

/**
 * The evaluator's core data: values, environments, and the shape of
 * `evaluate`. The book's evaluator works on list-structured expressions,
 * so an expression is itself just a `Value`: a symbol, a cons pair, or a
 * leaf. There is no separately parsed form; the syntax predicates of the
 * metacircular evaluator look at the data with the same car/cdr surgery
 * the book uses. The cons pairs come from `list.ts`, the mutable frame
 * chain from `env.ts`, and the checked failures from `errors.ts`. The
 * delayed arguments of section 4.2 are values too: the `ThunkValue` car
 * of the union is that section's machinery, added by `02-lazy.ts`.
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

/**
 * A delayed argument (section 4.2): the operand expression packaged with
 * the environment of the application that delayed it, the book's
 * `(thunk exp env)` list. Forcing evaluates the expression there. A
 * memoized thunk is the book's thunk-to-evaluated-thunk mutation: at the
 * first forcing the value replaces the expression in `exp`, and
 * `evaluated` records that the stored value answers every later forcing.
 * The thunks exercise 4.31 declares `lazy` never flip the flag, so they
 * re-evaluate at every demand.
 */
export interface ThunkValue {
  readonly _tag: "Thunk";
  /** The delayed expression; a memoized thunk stores its value here. */
  exp: Value;
  /** The environment of the application that delayed the operand. */
  readonly env: Env;
  /** Whether the value has been computed and stored in `exp`. */
  evaluated: boolean;
  /** Whether forcing stores the computed value; the section's thunks do. */
  readonly memoized: boolean;
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
  | ExecutionValue
  | ThunkValue;

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
