// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

/**
 * Runtime values (host-subsets grammar sections 3 and 4). Object variants are
 * engine classes dispatched by `instanceof`, never by a guest-visible field:
 * a guest record may legally carry a `tag` field of its own, so engine
 * identity cannot live in one. Guest records, arrays, maps, and sets hold
 * their data in these class instances; pair/list teaching data are ordinary
 * guest records with `tag` fields `cons`/`nil` (grammar section 3), built by
 * the `makePair`/`makeNil` helpers below. Dense arrays are mutable and
 * ordered; `Map`/`Set` keep native insertion order and key equality.
 */
import type { Block, Expr } from "../syntax/ast.ts";
import type { Env } from "./env.ts";
import type { Outcome } from "./errors.ts";

/** A guest procedure: parameters, body, and the lexical environment it closed over. */
export class Closure {
  readonly tag = "closure";
  readonly params: ReadonlyArray<string>;
  readonly rest: string | null;
  readonly body: Block;
  readonly env: Env;

  constructor(params: ReadonlyArray<string>, rest: string | null, body: Block, env: Env) {
    this.params = params;
    this.rest = rest;
    this.body = body;
    this.env = env;
  }
}

/** A host primitive installed in the guest builtin table. */
export class PrimitiveProcedure {
  readonly tag = "primitive";
  readonly name: string;
  readonly fn: (args: ReadonlyArray<Value>) => Outcome;

  constructor(name: string, fn: (args: ReadonlyArray<Value>) => Outcome) {
    this.name = name;
    this.fn = fn;
  }
}

/** A dense, mutable, ordered guest array. */
export class ArrayValue {
  readonly tag = "array";
  readonly items: Value[];

  constructor(items: Value[]) {
    this.items = items;
  }
}

/** A guest data record: statically named fields with their values. */
export class RecordValue {
  readonly tag = "record";
  readonly fields: Map<string, Value>;
  readonly readonlyFields: ReadonlySet<string>;

  constructor(fields: Map<string, Value>, readonlyFields: ReadonlySet<string>) {
    this.fields = fields;
    this.readonlyFields = readonlyFields;
  }
}

/** A guest `Map`: insertion-ordered entries with SameValueZero keys. */
export class MapValue {
  readonly tag = "map";
  readonly entries: Map<Value, Value>;

  constructor(entries: Map<Value, Value>) {
    this.entries = entries;
  }
}

/** A guest `Set`: insertion-ordered unique items. */
export class SetValue {
  readonly tag = "set";
  readonly items: Set<Value>;

  constructor(items: Set<Value>) {
    this.items = items;
  }
}

/** A delayed computation of the named lazy experiments. */
export class ThunkValue {
  readonly tag = "thunk";
  readonly expr: Expr;
  readonly env: Env;
  computed: Value | undefined;
  evaluated: boolean;

  constructor(expr: Expr, env: Env) {
    this.expr = expr;
    this.env = env;
    this.computed = undefined;
    this.evaluated = false;
  }
}

/** A guest `Error` instance. */
export class ErrorValue {
  readonly tag = "error-value";
  readonly message: string;

  constructor(message: string) {
    this.message = message;
  }
}

/** Everything the evaluator reads or produces. */
export type Value =
  | number
  | string
  | boolean
  | null
  | undefined
  | Closure
  | PrimitiveProcedure
  | ArrayValue
  | RecordValue
  | MapValue
  | SetValue
  | ThunkValue
  | ErrorValue;

/** Builds a closure. */
export const makeClosure = (
  params: ReadonlyArray<string>,
  rest: string | null,
  body: Block,
  env: Env,
): Closure => new Closure(params, rest, body, env);

/** Builds a host primitive. */
export const makePrimitive = (
  name: string,
  fn: (args: ReadonlyArray<Value>) => Outcome,
): PrimitiveProcedure => new PrimitiveProcedure(name, fn);

/** Builds a dense array value from a sequence of items. */
export const makeArray = (items: ReadonlyArray<Value>): ArrayValue => new ArrayValue([...items]);

/** Builds a record value from ordered field entries. */
export const makeRecord = (
  fields: ReadonlyArray<readonly [string, Value]>,
  readonlyFields: ReadonlySet<string> = new Set(),
): RecordValue => new RecordValue(new Map(fields), readonlyFields);

/** Builds a map value from ordered entries. */
export const makeMap = (entries: ReadonlyArray<readonly [Value, Value]> = []): MapValue =>
  new MapValue(new Map(entries));

/** Builds a set value from items. */
export const makeSet = (items: ReadonlyArray<Value> = []): SetValue => new SetValue(new Set(items));

/** Builds a guest error value. */
export const makeErrorValue = (message: string): ErrorValue => new ErrorValue(message);

/** Whether the value is a guest closure. */
export const isClosure = (value: Value): value is Closure => value instanceof Closure;
/** Whether the value is a host primitive. */
export const isPrimitive = (value: Value): value is PrimitiveProcedure =>
  value instanceof PrimitiveProcedure;
/** Whether the value is a guest array. */
export const isArrayValue = (value: Value): value is ArrayValue => value instanceof ArrayValue;
/** Whether the value is a guest record. */
export const isRecordValue = (value: Value): value is RecordValue => value instanceof RecordValue;
/** Whether the value is a guest map. */
export const isMapValue = (value: Value): value is MapValue => value instanceof MapValue;
/** Whether the value is a guest set. */
export const isSetValue = (value: Value): value is SetValue => value instanceof SetValue;
/** Whether the value is a lazy thunk. */
export const isThunkValue = (value: Value): value is ThunkValue => value instanceof ThunkValue;
/** Whether the value is a guest error. */
export const isErrorValue = (value: Value): value is ErrorValue => value instanceof ErrorValue;

/** Builds pair data as a guest record (`tag: "cons"`, grammar section 3). */
export const makePair = (head: Value, tail: Value): RecordValue =>
  makeRecord([
    ["tag", "cons"],
    ["head", head],
    ["tail", tail],
  ]);

/** Builds the empty-list marker as a guest record (`tag: "nil"`). */
export const makeNil = (): RecordValue => makeRecord([["tag", "nil"]]);

/** Whether the value is pair data built by `makePair` or a lookalike guest record. */
export const isPairValue = (value: Value): value is RecordValue =>
  isRecordValue(value) && value.fields.get("tag") === "cons";

/** Whether the value is the empty-list marker record. */
export const isNilValue = (value: Value): boolean =>
  isRecordValue(value) && value.fields.get("tag") === "nil";

/** The head of pair data, or `undefined` for anything else. */
export const pairHead = (value: Value): Value =>
  isPairValue(value) ? value.fields.get("head") : undefined;

/** The tail of pair data, or `undefined` for anything else. */
export const pairTail = (value: Value): Value =>
  isPairValue(value) ? value.fields.get("tail") : undefined;
