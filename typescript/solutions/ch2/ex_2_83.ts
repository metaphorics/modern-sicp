// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { attachTag, type Tagged } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithContents,
  type ArithDatum,
  applyGeneric,
  type GenError,
  makeComplexFromRealImag,
  makeRational,
  makeSchemeNumber,
  op,
  put,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.83: a generic `raise` for the tower of Figure 2.25. Each
 * level but the top installs a one-step entry: ordinary numbers rise
 * to rationals, rationals to reals, reals to complex numbers. The
 * complex level has no raise, so a raise past the top is the book's
 * "No method for these types".
 */

const ratContents = (c: ArithContents): [bigint, bigint] | undefined =>
  Array.isArray(c) && typeof c[0] === "bigint" && typeof c[1] === "bigint"
    ? [c[0], c[1]]
    : undefined;

const realContents = (c: ArithContents): number | undefined =>
  typeof c === "number" ? c : undefined;

/** Installs the three raise entries in the section's operation table.
 * Idempotent: installing again overwrites. */
export const installRaise = (): void => {
  put(
    "raise",
    ["scheme-number"],
    op((args) => {
      const x = args[0];
      return typeof x === "bigint" ? ok(makeRational(x, 1n)) : missRaise("scheme-number");
    }),
  );
  put(
    "raise",
    ["rational"],
    op((args) => {
      const x = args[0];
      const c = x !== undefined ? ratContents(x) : undefined;
      return c !== undefined
        ? ok(attachTag("real", Number(c[0]) / Number(c[1])))
        : missRaise("rational");
    }),
  );
  put(
    "raise",
    ["real"],
    op((args) => {
      const x = args[0];
      const c = x !== undefined ? realContents(x) : undefined;
      return c !== undefined ? ok(makeComplexFromRealImag(c, 0)) : missRaise("real");
    }),
  );
};

const missRaise = (tag: string): Result<ArithDatum, GenError> => ({
  _tag: "Error",
  error: { _tag: "NoMethod", op: "raise", tags: [tag] },
});

/** The generic raise: dispatch to the next level up, or fail at the
 * top of the tower. */
export const raise = (x: ArithDatum): Result<ArithDatum, GenError> => applyGeneric("raise", x);

/** The tower's constructors, for the tests. */
export const sn83 = (n: bigint): bigint => makeSchemeNumber(n);
export const rat83 = (n: bigint, d: bigint) => makeRational(n, d);
export const real83 = (x: number): Tagged<"real", number> => attachTag("real", x);
