// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

/**
 * The guest builtin table (host-subsets grammar sections 4 and 8): a real
 * operation table keyed by `<kind>.<member>`, the section 2.4 put/get lesson
 * carried into the evaluator. `get` on a missing key returns `undefined`, and
 * the caller tests before applying. Every handler is pure data over evaluated
 * arguments and returns the declared `Outcome`; no `Effect`, host I/O, or
 * boundary error lives here. Operations that must call guest procedures
 * (array `map`, `filter`, `reduce`, ...) stay in the evaluator, which owns
 * application; this table holds the first-order admitted library members.
 */
import { fail, type Outcome, ok } from "./errors.ts";
import {
  isArrayValue,
  isMapValue,
  isSetValue,
  makeArray,
  makePrimitive,
  RecordValue,
  type Value,
} from "./value.ts";

/** A builtin: evaluated arguments in, checked outcome out. */
export type Builtin = (args: ReadonlyArray<Value>) => Outcome;

/** The operation table of installed builtin handlers. */
export class OpTable {
  readonly #ops: Map<string, Builtin> = new Map();

  /** Installs or replaces the handler for `name`. */
  put(name: string, fn: Builtin): void {
    this.#ops.set(name, fn);
  }

  /** The handler for `name`, or `undefined` when absent. */
  get(name: string): Builtin | undefined {
    return this.#ops.get(name);
  }
}

const bad = (operator: string, detail: string): Outcome =>
  fail({ tag: "bad-operand", operator, detail });

const asNumber = (value: Value): number | undefined =>
  typeof value === "number" ? value : undefined;

const numberAt = (args: ReadonlyArray<Value>, index: number): number | undefined => {
  const value = args[index];
  return value === undefined ? undefined : asNumber(value);
};

/** Builds the namespace record for `prefix` from the installed table. */
export const namespaceValue = (
  table: OpTable,
  prefix: string,
  names: ReadonlyArray<string>,
): RecordValue => {
  const fields = new Map<string, Value>();
  for (const name of names) {
    const fn = table.get(`${prefix}.${name}`);
    if (fn !== undefined) {
      fields.set(name, makePrimitive(`${prefix}.${name}`, fn));
    }
  }
  return new RecordValue(fields, new Set());
};

const flatten = (items: ReadonlyArray<Value>, depth: number): Value[] => {
  const out: Value[] = [];
  for (const item of items) {
    if (isArrayValue(item) && depth > 0) {
      out.push(...flatten(item.items, depth - 1));
      continue;
    }
    out.push(item);
  }
  return out;
};

const installArrayBuiltins = (table: OpTable): void => {
  table.put("array.push", (args) => {
    const receiver = args[0];
    if (!isArrayValue(receiver)) {
      return bad("push", "receiver is not an array");
    }
    receiver.items.push(...args.slice(1));
    return ok(receiver.items.length);
  });
  table.put("array.pop", (args) => {
    const receiver = args[0];
    if (!isArrayValue(receiver)) {
      return bad("pop", "receiver is not an array");
    }
    return ok(receiver.items.pop());
  });
  table.put("array.slice", (args) => {
    const receiver = args[0];
    if (!isArrayValue(receiver)) {
      return bad("slice", "receiver is not an array");
    }
    return ok(makeArray(receiver.items.slice(numberAt(args, 1) ?? 0, numberAt(args, 2))));
  });
  table.put("array.flat", (args) => {
    const receiver = args[0];
    if (!isArrayValue(receiver)) {
      return bad("flat", "receiver is not an array");
    }
    return ok(makeArray(flatten(receiver.items, numberAt(args, 1) ?? 1)));
  });
  table.put("array.includes", (args) => {
    const receiver = args[0];
    if (!isArrayValue(receiver)) {
      return bad("includes", "receiver is not an array");
    }
    return ok(receiver.items.some((item) => Object.is(item, args[1])));
  });
  table.put("array.join", (args) => {
    const receiver = args[0];
    if (!isArrayValue(receiver)) {
      return bad("join", "receiver is not an array");
    }
    const separator = args[1] === undefined ? "," : String(args[1]);
    return ok(
      receiver.items
        .map((item) => (item === null || item === undefined ? "" : String(item)))
        .join(separator),
    );
  });
  table.put("array.reverse", (args) => {
    const receiver = args[0];
    if (!isArrayValue(receiver)) {
      return bad("reverse", "receiver is not an array");
    }
    receiver.items.reverse();
    return ok(receiver);
  });
  table.put("array.at", (args) => {
    const receiver = args[0];
    const index = numberAt(args, 1);
    if (!isArrayValue(receiver) || index === undefined) {
      return bad("at", "receiver is not an array or index is not a number");
    }
    return ok(receiver.items.at(index));
  });
};

const installStringBuiltins = (table: OpTable): void => {
  table.put("string.slice", (args) => {
    const receiver = args[0];
    if (typeof receiver !== "string") {
      return bad("slice", "receiver is not a string");
    }
    return ok(receiver.slice(numberAt(args, 1) ?? 0, numberAt(args, 2)));
  });
  table.put("string.split", (args) => {
    const receiver = args[0];
    const separator = args[1];
    if (typeof receiver !== "string" || typeof separator !== "string") {
      return bad("split", "receiver or separator is not a string");
    }
    return ok(makeArray(receiver.split(separator)));
  });
  table.put("string.startsWith", (args) => {
    const receiver = args[0];
    const search = args[1];
    if (typeof receiver !== "string" || typeof search !== "string") {
      return bad("startsWith", "receiver or search value is not a string");
    }
    return ok(receiver.startsWith(search));
  });
  table.put("string.includes", (args) => {
    const receiver = args[0];
    const search = args[1];
    if (typeof receiver !== "string" || typeof search !== "string") {
      return bad("includes", "receiver or search value is not a string");
    }
    return ok(receiver.includes(search));
  });
  table.put("string.replaceAll", (args) => {
    const receiver = args[0];
    const search = args[1];
    const replacement = args[2];
    if (
      typeof receiver !== "string" ||
      typeof search !== "string" ||
      typeof replacement !== "string"
    ) {
      return bad("replaceAll", "receiver, search, and replacement must be strings");
    }
    return ok(receiver.replaceAll(search, replacement));
  });
  table.put("string.padEnd", (args) => {
    const receiver = args[0];
    const length = numberAt(args, 1);
    const fill = args[2];
    if (typeof receiver !== "string" || length === undefined) {
      return bad("padEnd", "receiver is not a string or length is not a number");
    }
    return ok(receiver.padEnd(length, typeof fill === "string" ? fill : undefined));
  });
  table.put("string.trimEnd", (args) => {
    const receiver = args[0];
    if (typeof receiver !== "string") {
      return bad("trimEnd", "receiver is not a string");
    }
    return ok(receiver.trimEnd());
  });
  table.put("string.at", (args) => {
    const receiver = args[0];
    const index = numberAt(args, 1);
    if (typeof receiver !== "string" || index === undefined) {
      return bad("at", "receiver is not a string or index is not a number");
    }
    return ok(receiver.at(index));
  });
};

const installCollectionBuiltins = (table: OpTable): void => {
  table.put("map.get", (args) => {
    const receiver = args[0];
    return isMapValue(receiver)
      ? ok(receiver.entries.get(args[1]))
      : bad("get", "receiver is not a map");
  });
  table.put("map.set", (args) => {
    const receiver = args[0];
    if (!isMapValue(receiver)) {
      return bad("set", "receiver is not a map");
    }
    receiver.entries.set(args[1], args[2]);
    return ok(receiver);
  });
  table.put("map.has", (args) => {
    const receiver = args[0];
    return isMapValue(receiver)
      ? ok(receiver.entries.has(args[1]))
      : bad("has", "receiver is not a map");
  });
  table.put("map.delete", (args) => {
    const receiver = args[0];
    return isMapValue(receiver)
      ? ok(receiver.entries.delete(args[1]))
      : bad("delete", "receiver is not a map");
  });
  table.put("map.entries", (args) => {
    const receiver = args[0];
    if (!isMapValue(receiver)) {
      return bad("entries", "receiver is not a map");
    }
    return ok(makeArray([...receiver.entries].map(([key, value]) => makeArray([key, value]))));
  });
  table.put("map.keys", (args) => {
    const receiver = args[0];
    return isMapValue(receiver)
      ? ok(makeArray([...receiver.entries.keys()]))
      : bad("keys", "receiver is not a map");
  });
  table.put("map.values", (args) => {
    const receiver = args[0];
    return isMapValue(receiver)
      ? ok(makeArray([...receiver.entries.values()]))
      : bad("values", "receiver is not a map");
  });
  table.put("set.add", (args) => {
    const receiver = args[0];
    if (!isSetValue(receiver)) {
      return bad("add", "receiver is not a set");
    }
    receiver.items.add(args[1]);
    return ok(receiver);
  });
  table.put("set.has", (args) => {
    const receiver = args[0];
    return isSetValue(receiver)
      ? ok(receiver.items.has(args[1]))
      : bad("has", "receiver is not a set");
  });
  table.put("set.delete", (args) => {
    const receiver = args[0];
    return isSetValue(receiver)
      ? ok(receiver.items.delete(args[1]))
      : bad("delete", "receiver is not a set");
  });
  table.put("set.values", (args) => {
    const receiver = args[0];
    return isSetValue(receiver)
      ? ok(makeArray([...receiver.items]))
      : bad("values", "receiver is not a set");
  });
};

const installMathBuiltins = (table: OpTable): void => {
  const unary = (name: string, fn: (n: number) => number): void => {
    table.put(`math.${name}`, (args) => {
      const n = asNumber(args[0]);
      return n === undefined ? bad(name, "operand is not a number") : ok(fn(n));
    });
  };
  unary("abs", Math.abs);
  unary("floor", Math.floor);
  unary("sqrt", Math.sqrt);
  unary("trunc", Math.trunc);
  table.put("math.min", (args) => ok(Math.min(...args.map((arg) => asNumber(arg) ?? Number.NaN))));
  table.put("math.max", (args) => ok(Math.max(...args.map((arg) => asNumber(arg) ?? Number.NaN))));
  table.put("number.isInteger", (args) => ok(Number.isInteger(args[0])));
};

/** The complete admitted first-order builtin table. */
export const makeBuiltins = (): OpTable => {
  const table = new OpTable();
  installArrayBuiltins(table);
  installStringBuiltins(table);
  installCollectionBuiltins(table);
  installMathBuiltins(table);
  return table;
};

const builtinMethods = makeBuiltins();

/** Binds an installed first-order member to its guest receiver. */
export const builtinMember = (receiver: Value, name: string): Value | undefined => {
  const kind = isArrayValue(receiver)
    ? "array"
    : isMapValue(receiver)
      ? "map"
      : isSetValue(receiver)
        ? "set"
        : typeof receiver === "string"
          ? "string"
          : undefined;
  const method = kind === undefined ? undefined : builtinMethods.get(`${kind}.${name}`);
  return method === undefined
    ? undefined
    : makePrimitive(`${kind}.${name}`, (args) => method([receiver, ...args]));
};

/** Names admitted on the `Math` namespace value. */
export const MATH_NAMES: ReadonlyArray<string> = ["abs", "floor", "max", "min", "sqrt", "trunc"];
/** Names admitted on the `Number` namespace value. */
export const NUMBER_NAMES: ReadonlyArray<string> = ["isInteger"];
