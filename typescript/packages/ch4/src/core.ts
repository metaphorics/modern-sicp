// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

/**
 * The evaluator's core data: expressions, values, environments, and the shape
 * of `evaluate`. The metacircular evaluator itself is section 4.1's lesson;
 * the spine fixes the types every chapter 4 and 5 module shares. The `Thunk`
 * value and its memoizing constructor serve section 4.2's lazy evaluation.
 */
import { Effect, type HashMap, Option, Ref } from "effect";

import type { EvaluationError } from "./errors.js";
import type { List } from "./list.js";

export interface SelfEvaluating {
  readonly _tag: "SelfEvaluating";
  readonly value: number | bigint | string | boolean;
}

export interface SymbolExpr {
  readonly _tag: "Symbol";
  readonly name: string;
}

export interface QuoteExpr {
  readonly _tag: "Quote";
  readonly datum: Value;
}

export interface IfExpr {
  readonly _tag: "If";
  readonly predicate: Expr;
  readonly consequent: Expr;
  readonly alternative: Expr;
}

export interface LambdaExpr {
  readonly _tag: "Lambda";
  readonly params: List<string>;
  readonly body: ReadonlyArray<Expr>;
}

export interface BeginExpr {
  readonly _tag: "Begin";
  readonly actions: ReadonlyArray<Expr>;
}

/** A `cond` clause; the book's `else` is its own variant. */
export type CondClause =
  | { readonly _tag: "Clause"; readonly test: Expr; readonly body: ReadonlyArray<Expr> }
  | { readonly _tag: "Else"; readonly body: ReadonlyArray<Expr> };

export interface CondExpr {
  readonly _tag: "Cond";
  readonly clauses: ReadonlyArray<CondClause>;
}

export interface LetBinding {
  readonly name: string;
  readonly value: Expr;
}

export interface LetExpr {
  readonly _tag: "Let";
  readonly bindings: ReadonlyArray<LetBinding>;
  readonly body: ReadonlyArray<Expr>;
}

export interface DefineExpr {
  readonly _tag: "Define";
  readonly name: string;
  readonly value: Expr;
}

export interface SetExpr {
  readonly _tag: "Set";
  readonly name: string;
  readonly value: Expr;
}

export interface ApplicationExpr {
  readonly _tag: "Application";
  readonly operator: Expr;
  readonly operands: ReadonlyArray<Expr>;
}

/** The Scheme subset of the book's chapter 4, exhaustive over `_tag`. */
export type Expr =
  | SelfEvaluating
  | SymbolExpr
  | QuoteExpr
  | IfExpr
  | LambdaExpr
  | BeginExpr
  | CondExpr
  | LetExpr
  | DefineExpr
  | SetExpr
  | ApplicationExpr;

/** A primitive: applied to evaluated argument values, it yields one value. */
export type Primitive = (args: ReadonlyArray<Value>) => Effect.Effect<Value, EvaluationError>;

export interface NumberValue {
  readonly _tag: "Number";
  readonly n: number | bigint;
}

export interface BooleanValue {
  readonly _tag: "Boolean";
  readonly b: boolean;
}

export interface StringValue {
  readonly _tag: "String";
  readonly s: string;
}

export interface SymbolValue {
  readonly _tag: "Symbol";
  readonly name: string;
}

export interface ListValue {
  readonly _tag: "List";
  readonly items: List<Value>;
}

export interface PrimitiveValue {
  readonly _tag: "Primitive";
  readonly fn: Primitive;
}

/** A compound procedure: parameters, body, and the defining environment. */
export interface CompoundProc {
  readonly _tag: "Compound";
  readonly params: List<string>;
  readonly body: ReadonlyArray<Expr>;
  readonly env: Env;
}

/** A delayed computation (section 4.2) whose `force` memoizes into `cell`. */
export interface ThunkValue {
  readonly _tag: "Thunk";
  readonly cell: Ref.Ref<Option.Option<Value>>;
  readonly force: () => Effect.Effect<Value, EvaluationError>;
}

/** Everything the evaluator can produce. */
export type Value =
  | NumberValue
  | BooleanValue
  | StringValue
  | SymbolValue
  | ListValue
  | PrimitiveValue
  | CompoundProc
  | ThunkValue;

/**
 * The environment: one `Ref`-held `HashMap` frame plus the enclosing
 * environment; the global environment is the frame with no parent
 * (section 3.2 re-cut). The `Ref` is what makes `set!` real: every node
 * sharing the frame observes the write.
 */
export interface Env {
  readonly vars: Ref.Ref<HashMap.HashMap<string, Value>>;
  readonly parent: Option.Option<Env>;
}

/** `(expr, env) => Effect<Value, EvaluationError>` — the book's `eval`. */
export type Evaluate = (expr: Expr, env: Env) => Effect.Effect<Value, EvaluationError>;

/** Section 4.1.7's `analyze`: compile once, apply to many environments. */
export type Analyze = (expr: Expr) => (env: Env) => Effect.Effect<Value, EvaluationError>;

/**
 * Builds a memoized thunk, the edition's `delay` + `memo-proc`: the first
 * `force` runs `work` and stores the value in `cell`; every later `force`
 * reads the cell and never re-runs `work`.
 */
export const makeThunk = (
  work: () => Effect.Effect<Value, EvaluationError>,
): Effect.Effect<ThunkValue> =>
  Effect.gen(function* () {
    const cell: Ref.Ref<Option.Option<Value>> = yield* Ref.make(Option.none<Value>());
    return {
      _tag: "Thunk",
      cell,
      force: () =>
        Effect.gen(function* () {
          const cached = yield* Ref.get(cell);
          if (Option.isSome(cached)) {
            return cached.value;
          }
          const value = yield* work();
          yield* Ref.set(cell, Option.some(value));
          return value;
        }),
    };
  });
