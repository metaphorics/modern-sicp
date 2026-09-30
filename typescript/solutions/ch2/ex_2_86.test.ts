// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeRational, makeTsNumber, show } from "../../packages/ch2/src/05-generic-operations.js";
import { addComplex86, cosine, magnitude86, makeComplex86, sine, sqrt } from "./ex_2_86.js";

const sn = (n: bigint) => makeTsNumber(n);

describe("exercise 2.86: generic complex parts", () => {
  it("takes the magnitude of a whole-number complex exactly", () => {
    expect(show(magnitude86(makeComplex86(sn(3n), sn(4n))))).toBe("5");
  });

  it("takes the magnitude of a rational-parts complex exactly", () => {
    expect(show(magnitude86(makeComplex86(makeRational(3n, 1n), makeRational(4n, 1n))))).toBe(
      "[rational, 5, 1]",
    );
  });

  it("falls to an inexact real when exactness ends", () => {
    expect(show(magnitude86(makeComplex86(sn(1n), sn(1n))))).toBe(`[real, ${Math.SQRT2}]`);
  });

  it("adds generic-parts complex numbers through generic add", () => {
    const s = addComplex86(
      makeComplex86(sn(3n), makeRational(1n, 2n)),
      makeComplex86(sn(2n), makeRational(1n, 3n)),
    );
    expect(s._tag === "Ok" && s.value.contents.real).toBe(5n);
    expect(s._tag === "Ok" && s.value.contents.imag).toEqual(makeRational(5n, 6n));
  });

  it("answers generic sine and cosine", () => {
    expect(show(sine(sn(0n)))).toBe("[real, 0]");
    expect(show(cosine(makeRational(0n, 1n)))).toBe("[real, 1]");
  });

  it("keeps an exact square root exact across types", () => {
    expect(show(sqrt(sn(25n)))).toBe("5");
    expect(show(sqrt(makeRational(9n, 4n)))).toBe("[rational, 3, 2]");
  });
});
