// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { attachTag, type Tagged } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithDatum,
  contentsOf,
  type GenError,
  makeComplexFromRealImag,
  makeRational,
  makeTsNumber,
  typeTagOf,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.82: coercion in the general case of multiple arguments.
 * The strategy: pick each argument's type in turn as the target, bring
 * every argument there through the coercion table, and apply the
 * operation at the coerced tags. The rationale records the
 * counterexample: with only the section's one-step coercions, the
 * strategy never walks a two-step chain, so a `[real, real]` operation
 * is not tried for `[ts-number, rational]` even though both
 * arguments can reach real.
 */

/** This exercise's real level, the tower of Figure 2.25's third row. */
export type Real82 = Tagged<"real", number>;

export type Coercion = (d: ArithDatum) => Result<ArithDatum, GenError>;
export type MultiOp = (args: ReadonlyArray<ArithDatum>) => Result<ArithDatum, GenError>;

const coercions = new Map<string, Coercion>();

/** Installs the section's two one-step coercions: ordinary numbers
 * into rationals, rationals into reals. */
export const installCoercions82 = (): void => {
  coercions.set("ts-number>rational", (n) => {
    const c = contentsOf(n);
    return typeof c === "bigint"
      ? ok(makeRational(c, 1n))
      : err({ _tag: "NoMethod", op: "ts-number->rational", tags: [typeTagOf(n)] });
  });
  coercions.set("rational>real", (r) => {
    const c = contentsOf(r);
    return Array.isArray(c) && typeof c[0] === "bigint" && typeof c[1] === "bigint"
      ? ok(attachTag("real", Number(c[0]) / Number(c[1])))
      : err({ _tag: "NoMethod", op: "rational->real", tags: [typeTagOf(r)] });
  });
};

/** Brings one datum to the target type, or reports why it cannot. */
const coerceOne = (d: ArithDatum, target: string): Result<ArithDatum, GenError> => {
  const from = typeTagOf(d);
  if (from === target) {
    return ok(d);
  }
  const c = coercions.get(`${from}>${target}`);
  return c === undefined ? err({ _tag: "NoMethod", op: "coerce", tags: [from, target] }) : c(d);
};

/** The multi-argument dispatch: for each argument's type in turn,
 * coerce all the arguments there and apply the operation if one is
 * installed at the coerced tags. */
export const applyGenericMulti = (
  lookup: (tags: ReadonlyArray<string>) => MultiOp | undefined,
  ...args: ReadonlyArray<ArithDatum>
): Result<ArithDatum, GenError> => {
  for (const target of args.map(typeTagOf)) {
    const brought: ArithDatum[] = [];
    let reachable = true;
    for (const d of args) {
      const c = coerceOne(d, target);
      if (c._tag === "Error") {
        reachable = false;
        break;
      }
      brought.push(c.value);
    }
    if (reachable) {
      const proc = lookup(brought.map(typeTagOf));
      if (proc !== undefined) {
        return proc(brought);
      }
    }
  }
  return err({ _tag: "NoMethod", op: "apply-generic", tags: args.map(typeTagOf) });
};

/** The multiplication this exercise dispatches, defined only for two
 * rationals. */
export const mulRational82 = (args: ReadonlyArray<ArithDatum>): Result<ArithDatum, GenError> => {
  const a = args[0];
  const b = args[1];
  const ca = a !== undefined ? contentsOf(a) : undefined;
  const cb = b !== undefined ? contentsOf(b) : undefined;
  if (
    ca === undefined ||
    cb === undefined ||
    !Array.isArray(ca) ||
    !Array.isArray(cb) ||
    typeof ca[0] !== "bigint" ||
    typeof ca[1] !== "bigint" ||
    typeof cb[0] !== "bigint" ||
    typeof cb[1] !== "bigint"
  ) {
    return err({ _tag: "NoMethod", op: "mul", tags: ["rational", "rational"] });
  }
  return ok(makeRational(ca[0] * cb[0], ca[1] * cb[1]));
};

/** The addition this exercise dispatches, defined only for two
 * reals. */
export const addReal82 = (args: ReadonlyArray<ArithDatum>): Result<ArithDatum, GenError> => {
  const a = args[0];
  const b = args[1];
  const ca = a !== undefined ? contentsOf(a) : undefined;
  const cb = b !== undefined ? contentsOf(b) : undefined;
  if (typeof ca !== "number" || typeof cb !== "number") {
    return err({ _tag: "NoMethod", op: "add", tags: ["real", "real"] });
  }
  return ok(attachTag("real", ca + cb));
};

/** The tower's constructors, for the tests. */
export const real82 = (x: number): Real82 => attachTag("real", x);
export const sn82 = (n: bigint): bigint => makeTsNumber(n);
export const rat82 = (n: bigint, d: bigint) => makeRational(n, d);
export const cpx82 = (x: number, y: number) => makeComplexFromRealImag(x, y);
