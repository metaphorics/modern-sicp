// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  makeComplexFromMagAng,
  makeComplexFromRealImag,
  makeRational,
  makeTsNumber,
} from "../../packages/ch2/src/05-generic-operations.js";
import { equ79 } from "./ex_2_79.js";

describe("exercise 2.79: generic equ?", () => {
  it("answers for ordinary numbers", () => {
    expect(equ79(makeTsNumber(3n), makeTsNumber(3n))).toEqual({ _tag: "Ok", value: true });
    expect(equ79(makeTsNumber(3n), makeTsNumber(4n))).toEqual({ _tag: "Ok", value: false });
  });

  it("answers for rationals, 1/2 equal to 2/4", () => {
    expect(equ79(makeRational(1n, 2n), makeRational(2n, 4n))).toEqual({ _tag: "Ok", value: true });
    expect(equ79(makeRational(1n, 2n), makeRational(1n, 3n))).toEqual({ _tag: "Ok", value: false });
  });

  it("answers for complex numbers by equal parts", () => {
    expect(equ79(makeComplexFromRealImag(1, 2), makeComplexFromRealImag(1, 2))).toEqual({
      _tag: "Ok",
      value: true,
    });
    expect(
      equ79(makeComplexFromRealImag(1, 2), makeComplexFromMagAng(Math.sqrt(5), Math.atan2(2, 1))),
    ).toEqual({
      _tag: "Ok",
      value: false,
    });
  });

  it("has no entry for arguments of different types", () => {
    expect(equ79(makeTsNumber(3n), makeRational(3n, 1n))).toEqual({
      _tag: "Error",
      error: { _tag: "NoMethod", op: "equ?", tags: ["ts-number", "rational"] },
    });
  });
});
