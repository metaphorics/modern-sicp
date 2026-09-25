// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import type { Option } from "../../packages/ch2/src/02-picture-language.js";
import { get, makeOpTable, type OpTable, put } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithContents,
  type ArithDatum,
  contentsOf,
  type GenError,
  makeComplexFromRealImag,
  makeSchemeNumber,
  typeTagOf,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.81: Louis Reasoner's self-coercions. Part (a) installs
 * identity entries in the coercion table and shows where that sends
 * `apply-generic`; part (b) judges the design; part (c) is the guard:
 * when the arguments already share a type, coercion is not tried.
 * This file builds its own operation and coercion tables so Louis's
 * identities never leak into the section system.
 */

type Entry = (args: ReadonlyArray<ArithContents>) => Result<ArithDatum, GenError>;
type Coercion = (d: ArithDatum) => Result<ArithDatum, GenError>;

const table81: OpTable<Entry> = makeOpTable();
const coercion81: OpTable<Coercion> = makeOpTable();

const putOp81 = (op: string, tags: ReadonlyArray<string>, fn: Entry): void => {
  put(table81, op, tags, fn);
};

const putCoercion81 = (from: string, to: string, fn: Coercion): void => {
  put(coercion81, "coerce", [`${from}->${to}`], fn);
};

const getCoercion81 = (from: string, to: string): Option<Coercion> =>
  get(coercion81, "coerce", [`${from}->${to}`]);

const miss81 = (op: string, tags: ReadonlyArray<string>): Result<ArithDatum, GenError> => ({
  _tag: "Error",
  error: { _tag: "NoMethod", op, tags },
});

const bind81 = (
  r: Result<ArithDatum, GenError>,
  f: (d: ArithDatum) => Result<ArithDatum, GenError>,
): Result<ArithDatum, GenError> => (r._tag === "Ok" ? f(r.value) : r);

/** Reads the real coordinate of a complex contents, whatever inner
 * representation carries it. */
const repPart = (c: ArithContents): number | undefined => {
  if (typeof c !== "object" || c === null || Array.isArray(c) || !("_tag" in c)) {
    return undefined;
  }
  if (c._tag === "rectangular") {
    return c.contents[0];
  }
  if (c._tag === "polar") {
    return c.contents[0] * Math.cos(c.contents[1]);
  }
  return undefined;
};

/** Reads the imaginary coordinate of a complex contents. */
const repPart2 = (c: ArithContents): number | undefined => {
  if (typeof c !== "object" || c === null || Array.isArray(c) || !("_tag" in c)) {
    return undefined;
  }
  if (c._tag === "rectangular") {
    return c.contents[1];
  }
  if (c._tag === "polar") {
    return c.contents[0] * Math.sin(c.contents[1]);
  }
  return undefined;
};

// The section's entries: exponentiation for ordinary numbers only, and
// addition for ordinary numbers and for complex numbers.
putOp81("exp", ["scheme-number", "scheme-number"], (args) => {
  const x = args[0];
  const y = args[1];
  return typeof x === "bigint" && typeof y === "bigint"
    ? ok(makeSchemeNumber(x ** y))
    : miss81("exp", ["scheme-number", "scheme-number"]);
});
putOp81("add", ["scheme-number", "scheme-number"], (args) => {
  const x = args[0];
  const y = args[1];
  return typeof x === "bigint" && typeof y === "bigint"
    ? ok(makeSchemeNumber(x + y))
    : miss81("add", ["scheme-number", "scheme-number"]);
});
putOp81("add", ["complex", "complex"], (args) => {
  const a = args[0];
  const b = args[1];
  const ax = a !== undefined ? repPart(a) : undefined;
  const ay = a !== undefined ? repPart2(a) : undefined;
  const bx = b !== undefined ? repPart(b) : undefined;
  const by = b !== undefined ? repPart2(b) : undefined;
  return ax !== undefined && ay !== undefined && bx !== undefined && by !== undefined
    ? ok(makeComplexFromRealImag(ax + bx, ay + by))
    : miss81("add", ["complex", "complex"]);
});

// The ordinary-number to complex coercion of the section.
putCoercion81("scheme-number", "complex", (n) => {
  const c = contentsOf(n);
  return typeof c === "bigint"
    ? ok(makeComplexFromRealImag(Number(c), 0))
    : miss81("scheme-number->complex", ["scheme-number"]);
});

/** Part (a): Louis's identity self-coercions, installed under each
 * type back to itself. */
export const installSelfCoercions81 = (): void => {
  putCoercion81("scheme-number", "scheme-number", (n) => ok(n));
  putCoercion81("complex", "complex", (z) => ok(z));
};

const getOp81 = (op: string, tags: ReadonlyArray<string>): Option<Entry> => get(table81, op, tags);

/** The book's apply-generic with Louis's table: try the operation;
 * then, for two arguments, try coercing the first toward the second,
 * then the second toward the first. With identity self-coercions
 * installed and a same-type operation missing, the retry re-creates
 * the very call that failed --- the loop part (a) asks about. Never
 * call this on a same-type miss once the identities are in. */
export const applyGenericLouis = (
  opName: string,
  ...args: ReadonlyArray<ArithDatum>
): Result<ArithDatum, GenError> => {
  const tags = args.map(typeTagOf);
  const proc = getOp81(opName, tags);
  if (proc._tag === "Some") {
    return proc.value(args.map(contentsOf));
  }
  if (args.length === 2 && args[0] !== undefined && args[1] !== undefined) {
    const [a1, a2] = args;
    const type1 = tags[0] ?? "";
    const type2 = tags[1] ?? "";
    const t1toT2 = getCoercion81(type1, type2);
    if (t1toT2._tag === "Some") {
      return bind81(t1toT2.value(a1), (coerced) => applyGenericLouis(opName, coerced, a2));
    }
    const t2toT1 = getCoercion81(type2, type1);
    if (t2toT1._tag === "Some") {
      return bind81(t2toT1.value(a2), (coerced) => applyGenericLouis(opName, a1, coerced));
    }
  }
  return miss81(opName, tags);
};

/** Part (c): the guard. When the arguments already share a type the
 * coercion table is not consulted, so identity self-coercions cannot
 * send the dispatch in circles. */
export const applyGenericGuarded = (
  opName: string,
  ...args: ReadonlyArray<ArithDatum>
): Result<ArithDatum, GenError> => {
  const tags = args.map(typeTagOf);
  const proc = getOp81(opName, tags);
  if (proc._tag === "Some") {
    return proc.value(args.map(contentsOf));
  }
  if (tags.length === 2 && tags[0] === tags[1]) {
    return miss81(opName, tags);
  }
  if (args.length === 2 && args[0] !== undefined && args[1] !== undefined) {
    const [a1, a2] = args;
    const type1 = tags[0] ?? "";
    const type2 = tags[1] ?? "";
    const t1toT2 = getCoercion81(type1, type2);
    if (t1toT2._tag === "Some") {
      return bind81(t1toT2.value(a1), (coerced) => applyGenericGuarded(opName, coerced, a2));
    }
    const t2toT1 = getCoercion81(type2, type1);
    if (t2toT1._tag === "Some") {
      return bind81(t2toT1.value(a2), (coerced) => applyGenericGuarded(opName, a1, coerced));
    }
  }
  return miss81(opName, tags);
};

/** Part (b)'s cross-type operation, through the guarded dispatch. */
export const add81 = (x: ArithDatum, y: ArithDatum): Result<ArithDatum, GenError> =>
  applyGenericGuarded("add", x, y);

/** Exponentiation, the operation only the ordinary-number package
 * defines. */
export const exp81 = (x: ArithDatum, y: ArithDatum): Result<ArithDatum, GenError> =>
  applyGenericGuarded("exp", x, y);

/** Reads a coercion entry, for the test that inspects the table. */
export const lookupCoercion81 = (from: string, to: string): Option<Coercion> =>
  getCoercion81(from, to);
