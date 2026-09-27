// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import type { Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { type GenError, makeSchemeNumber } from "../../packages/ch2/src/05-generic-operations.js";
import type { DenseList } from "./ex_2_89.js";
import {
  addTermsDense,
  adjoinTermDense,
  denseOf,
  firstTermDense,
  restTermsDense,
  showDense,
} from "./ex_2_89.js";

const sn = (n: bigint) => makeSchemeNumber(n);
const A = (): Result<DenseList, GenError> =>
  denseOf([
    [5n, sn(1n)],
    [4n, sn(2n)],
    [2n, sn(3n)],
    [1n, sn(-2n)],
    [0n, sn(-5n)],
  ]);

describe("exercise 2.89: dense term lists", () => {
  it("represents the book's A as six coefficients", () => {
    const l = A();
    expect(l._tag === "Ok" && l.value.length).toBe(6);
    expect(l._tag === "Ok" && l.value).toEqual([sn(1n), sn(2n), sn(0n), sn(3n), sn(-2n), sn(-5n)]);
  });

  it("reads the first term and its order off the length", () => {
    const l = A();
    if (l._tag !== "Ok") {
      throw new Error("denseOf failed");
    }
    const t = firstTermDense(l.value);
    expect(t[0]).toBe(5n);
    expect(t[1]).toBe(sn(1n));
    const r = restTermsDense(l.value);
    expect(firstTermDense(r)[0]).toBe(4n);
  });

  it("adjoins with zero padding and skips zero coefficients", () => {
    const l = A();
    if (l._tag !== "Ok") {
      throw new Error("denseOf failed");
    }
    const wider = adjoinTermDense([7n, sn(4n)], l.value);
    expect(wider._tag === "Ok" && wider.value).toEqual([
      sn(4n),
      sn(0n),
      sn(1n),
      sn(2n),
      sn(0n),
      sn(3n),
      sn(-2n),
      sn(-5n),
    ]);
    const unchanged = adjoinTermDense([6n, sn(0n)], l.value);
    expect(unchanged._tag === "Ok" && unchanged.value).toEqual(l.value);
  });

  it("adds two dense polynomials coefficientwise", () => {
    const a = A();
    const b = denseOf([
      [2n, sn(1n)],
      [0n, sn(3n)],
    ]);
    if (a._tag !== "Ok" || b._tag !== "Ok") {
      throw new Error("denseOf failed");
    }
    const sum = addTermsDense(a.value, b.value);
    expect(sum._tag === "Ok" && showDense("x", sum.value)).toBe(
      "(polynomial x (5 1) (4 2) (2 4) (1 -2) (0 -2))",
    );
  });
});
