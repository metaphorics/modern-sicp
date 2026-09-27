// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { attachTag } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithContents,
  type ArithDatum,
  applyGeneric,
  type GenError,
  op,
  put,
  typeTagOf,
} from "../../packages/ch2/src/05-generic-operations.js";
import { installRaise, raise } from "./ex_2_83.js";

/**
 * Exercise 2.84: coercion by successive raising. To apply an operation
 * to arguments of different types, the lower argument is raised until
 * its type matches the other's --- or the attempt runs off the top of
 * the tower, which is the failure report. Which of two types is higher
 * is never looked up in a table: a type is above another exactly when
 * a chain of raises reaches it, so adding a new level means installing
 * its raise and nothing else.
 */

const realSum = (args: ReadonlyArray<ArithContents>): Result<ArithDatum, GenError> => {
  const x = args[0];
  const y = args[1];
  return typeof x === "number" && typeof y === "number"
    ? ok(attachTag("real", x + y))
    : err({ _tag: "NoMethod", op: "add", tags: ["real", "real"] });
};

/** Installs everything this exercise's dispatch needs: the raise
 * entries of 2.83 and a real-level add, so an operation exists at the
 * level the raising reaches. Idempotent. */
export const installTower84 = (): void => {
  installRaise();
  put("add", ["real", "real"], op(realSum));
};

/** Raises `x` repeatedly until it carries `target`, or fails when the
 * tower tops out first. */
export const raiseToward = (x: ArithDatum, target: string): Result<ArithDatum, GenError> => {
  if (typeTagOf(x) === target) {
    return ok(x);
  }
  const up = raise(x);
  return up._tag === "Error" ? up : raiseToward(up.value, target);
};

const miss84 = (op: string, tags: ReadonlyArray<string>): Result<ArithDatum, GenError> => ({
  _tag: "Error",
  error: { _tag: "NoMethod", op, tags },
});

/** The apply-generic of this exercise: same types dispatch directly;
 * otherwise the lower argument is raised toward the higher until the
 * types agree. */
export const applyGenericRaising = (
  opName: string,
  x: ArithDatum,
  y: ArithDatum,
): Result<ArithDatum, GenError> => {
  const tx = typeTagOf(x);
  const ty = typeTagOf(y);
  if (tx === ty) {
    return applyGeneric(opName, x, y);
  }
  const xUp = raiseToward(x, ty);
  if (xUp._tag === "Ok") {
    return applyGenericRaising(opName, xUp.value, y);
  }
  const yUp = raiseToward(y, tx);
  if (yUp._tag === "Ok") {
    return applyGenericRaising(opName, x, yUp.value);
  }
  return miss84(opName, [tx, ty]);
};
