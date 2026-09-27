// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeOpTable } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithHandler,
  addTower,
  applyTowerOp,
  type CoercionTable,
  installIntegerPackage,
  installRationalPackage,
  installRealPackage,
  mulTower,
  type TowerDatum,
  type TowerTable,
} from "./ex_2_76.js";

const integer = (n: number): TowerDatum => ({ _tag: "integer", contents: n });
const rational = (n: bigint, d: bigint): TowerDatum => ({
  _tag: "rational",
  contents: [n, d],
});
const real = (x: number): TowerDatum => ({ _tag: "real", contents: x });

describe("exercise 2.76: adding types versus adding operations", () => {
  it("adds two datums of the same type without coercion", () => {
    expect(addTower(integer(3), integer(4))).toEqual({
      _tag: "Ok",
      value: { _tag: "integer", contents: 7 },
    });
    expect(addTower(rational(1n, 2n), rational(1n, 3n))).toEqual({
      _tag: "Ok",
      value: { _tag: "rational", contents: [5n, 6n] },
    });
    expect(addTower(real(0.25), real(0.5))).toEqual({
      _tag: "Ok",
      value: { _tag: "real", contents: 0.75 },
    });
  });

  it("coerces the integer up to rational when the types differ", () => {
    expect(addTower(integer(3), rational(1n, 2n))).toEqual({
      _tag: "Ok",
      value: { _tag: "rational", contents: [7n, 2n] },
    });
  });

  it("coerces the rational up to real when the types differ", () => {
    expect(addTower(rational(1n, 2n), real(0.25))).toEqual({
      _tag: "Ok",
      value: { _tag: "real", contents: 0.75 },
    });
  });

  it("walks the integer-rational-real chain two steps when needed", () => {
    expect(addTower(integer(3), real(0.5))).toEqual({
      _tag: "Ok",
      value: { _tag: "real", contents: 3.5 },
    });
  });

  it("multiplication went in as three more entries, not a rewrite", () => {
    expect(mulTower(integer(3), rational(1n, 2n))).toEqual({
      _tag: "Ok",
      value: { _tag: "rational", contents: [3n, 2n] },
    });
    expect(mulTower(rational(1n, 2n), real(0.25))).toEqual({
      _tag: "Ok",
      value: { _tag: "real", contents: 0.125 },
    });
  });

  it("an operation with no handler for the types reports the book's error", () => {
    const missed = applyTowerOp("expt", integer(2), integer(10));
    expect(missed).toEqual({
      _tag: "Error",
      error: { _tag: "NoMethod", op: "expt", tags: ["integer", "integer"] },
    });
  });

  it("installing a later package leaves earlier handlers untouched, by reference", () => {
    const arith: TowerTable = makeOpTable();
    const coercions: CoercionTable = makeOpTable();
    installRationalPackage(arith, coercions);
    const before: ArithHandler | undefined = arith.get("add")?.get("rational,rational");
    installIntegerPackage(arith, coercions);
    installRealPackage(arith);
    // The rational package's handlers are the very same values after
    // two more installs: adding a type added entries, not rewrites.
    expect(arith.get("add")?.get("rational,rational")).toBe(before);
    expect(coercions.get("coerce")?.get("rational")).toBeDefined();
  });
});
