// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  makeComplexFromRealImag,
  makeRational,
  makeTsNumber,
} from "../../packages/ch2/src/05-generic-operations.js";
import { isZero80 } from "./ex_2_80.js";

describe("exercise 2.80: generic =zero?", () => {
  it("answers for ordinary numbers", () => {
    expect(isZero80(makeTsNumber(0n))).toEqual({ _tag: "Ok", value: true });
    expect(isZero80(makeTsNumber(5n))).toEqual({ _tag: "Ok", value: false });
  });

  it("answers for rationals by numerator", () => {
    expect(isZero80(makeRational(0n, 5n))).toEqual({ _tag: "Ok", value: true });
    expect(isZero80(makeRational(1n, 5n))).toEqual({ _tag: "Ok", value: false });
  });

  it("answers for complex numbers by both parts", () => {
    expect(isZero80(makeComplexFromRealImag(0, 0))).toEqual({ _tag: "Ok", value: true });
    expect(isZero80(makeComplexFromRealImag(0, 2))).toEqual({ _tag: "Ok", value: false });
  });
});
