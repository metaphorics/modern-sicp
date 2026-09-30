// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  addRat,
  err,
  makeRat,
  mulRat,
  ok,
  type Rat,
  type Result,
} from "../../packages/ch2/src/01-data-abstraction.js";
import { none, type Option, some } from "../../packages/ch2/src/02-picture-language.js";
import { get, makeOpTable, type OpTable, put } from "../../packages/ch2/src/04-data-directed.js";

/**
 * Exercise 2.76: adding types versus adding operations. This edition's
 * answer mounts the book's coercion tower on the section's table: the
 * three numeric representations --- integer, rational, real --- install
 * same-type arithmetic, and `applyTowerOp` extends the generic dispatch with
 * the tower's coercions, raising an argument step by step from integer
 * through rational to real until a handler matches. Adding the integer
 * type touched nothing installed before it; the `mul` operation was
 * three more table entries over the same packages. The strategy
 * comparison the book asks for is recorded in the rationale.
 */

/** A tower datum: the book's integer, rational, and real numbers, each
 * tagged with its type. */
export type TowerDatum =
  | { readonly _tag: "integer"; readonly contents: number }
  | { readonly _tag: "rational"; readonly contents: Rat }
  | { readonly _tag: "real"; readonly contents: number };

/** Why a tower operation could not answer: the book's "No method for
 * these types", carrying the operation and the two tags. */
export type TowerError = {
  readonly _tag: "NoMethod";
  readonly op: string;
  readonly tags: [string, string];
};

/** A same-type arithmetic handler over two tagged datums. */
export type ArithHandler = (a: TowerDatum, b: TowerDatum) => Result<TowerDatum, TowerError>;

/** A coercion: one datum raised one type up the tower, or nothing when
 * the datum is not the source type. */
export type CoercionHandler = (d: TowerDatum) => Option<TowerDatum>;

/** The tower's operation-and-type table, keyed by operation and the
 * ordered pair of argument tags. */
export type TowerTable = OpTable<ArithHandler>;

/** The coercion table, keyed by source type: the tower is linear, so
 * the one installed coercion per type raises it one level up. */
export type CoercionTable = OpTable<CoercionHandler>;

const noMethod = (op: string, a: TowerDatum, b: TowerDatum): TowerError => ({
  _tag: "NoMethod",
  op,
  tags: [a._tag, b._tag],
});

/** Installs the integer package: same-type add and mul, plus the
 * one-step coercion up to rational. */
export const installIntegerPackage = (arith: TowerTable, coercions: CoercionTable): void => {
  put(arith, "add", ["integer", "integer"], (a, b) =>
    a._tag === "integer" && b._tag === "integer"
      ? ok({ _tag: "integer", contents: a.contents + b.contents })
      : err(noMethod("add", a, b)),
  );
  put(arith, "mul", ["integer", "integer"], (a, b) =>
    a._tag === "integer" && b._tag === "integer"
      ? ok({ _tag: "integer", contents: a.contents * b.contents })
      : err(noMethod("mul", a, b)),
  );
  put(coercions, "coerce", ["integer"], (d) =>
    d._tag === "integer"
      ? some({ _tag: "rational", contents: makeRat(BigInt(d.contents), 1n) })
      : none,
  );
};

/** Installs the rational package: same-type add and mul over the exact
 * rationals, plus the coercion up to real. */
export const installRationalPackage = (arith: TowerTable, coercions: CoercionTable): void => {
  put(arith, "add", ["rational", "rational"], (a, b) =>
    a._tag === "rational" && b._tag === "rational"
      ? ok({ _tag: "rational", contents: addRat(a.contents, b.contents) })
      : err(noMethod("add", a, b)),
  );
  put(arith, "mul", ["rational", "rational"], (a, b) =>
    a._tag === "rational" && b._tag === "rational"
      ? ok({ _tag: "rational", contents: mulRat(a.contents, b.contents) })
      : err(noMethod("mul", a, b)),
  );
  put(coercions, "coerce", ["rational"], (d) =>
    d._tag === "rational"
      ? some({ _tag: "real", contents: Number(d.contents[0]) / Number(d.contents[1]) })
      : none,
  );
};

/** Installs the real package: same-type add and mul. */
export const installRealPackage = (arith: TowerTable): void => {
  put(arith, "add", ["real", "real"], (a, b) =>
    a._tag === "real" && b._tag === "real"
      ? ok({ _tag: "real", contents: a.contents + b.contents })
      : err(noMethod("add", a, b)),
  );
  put(arith, "mul", ["real", "real"], (a, b) =>
    a._tag === "real" && b._tag === "real"
      ? ok({ _tag: "real", contents: a.contents * b.contents })
      : err(noMethod("mul", a, b)),
  );
};

/** The system of this answer: all three packages installed. */
export const towerTable: TowerTable = makeOpTable();
export const coercionTable: CoercionTable = makeOpTable();
installIntegerPackage(towerTable, coercionTable);
installRationalPackage(towerTable, coercionTable);
installRealPackage(towerTable);

/** Raises a datum one step up the tower, per the installed coercions. */
const raiseOne = (d: TowerDatum): Option<TowerDatum> => {
  const up = get(coercionTable, "coerce", [d._tag]);
  return up._tag === "Some" ? up.value(d) : none;
};

/** Raises a datum along the tower until its type is `target`, or
 * nothing when no chain of coercions gets there. */
export const raiseTo = (d: TowerDatum, target: string): Option<TowerDatum> => {
  let current: TowerDatum = d;
  while (current._tag !== target) {
    const next = raiseOne(current);
    if (next._tag === "None") {
      return none;
    }
    current = next.value;
  }
  return some(current);
};

const applyDirect = (
  op: string,
  a: TowerDatum,
  b: TowerDatum,
): Result<TowerDatum, TowerError> | undefined => {
  const proc = get(towerTable, op, [a._tag, b._tag]);
  return proc._tag === "Some" ? proc.value(a, b) : undefined;
};

/** Generic dispatch extended with the tower: the same-type handler first,
 * then an argument raised along the coercion chain until the types
 * agree, then the book's "No method for these types". */
export const applyTowerOp = (
  op: string,
  a: TowerDatum,
  b: TowerDatum,
): Result<TowerDatum, TowerError> => {
  const direct = applyDirect(op, a, b);
  if (direct !== undefined) {
    return direct;
  }
  const aRaised = raiseTo(a, b._tag);
  if (aRaised._tag === "Some") {
    const raised = applyDirect(op, aRaised.value, b);
    if (raised !== undefined) {
      return raised;
    }
  }
  const bRaised = raiseTo(b, a._tag);
  if (bRaised._tag === "Some") {
    const raised = applyDirect(op, a, bRaised.value);
    if (raised !== undefined) {
      return raised;
    }
  }
  return err(noMethod(op, a, b));
};

/** The generic addition over the tower. */
export const addTower = (a: TowerDatum, b: TowerDatum): Result<TowerDatum, TowerError> =>
  applyTowerOp("add", a, b);

/** The generic multiplication over the tower: the operation added to
 * the existing packages as three more table entries. */
export const mulTower = (a: TowerDatum, b: TowerDatum): Result<TowerDatum, TowerError> =>
  applyTowerOp("mul", a, b);
