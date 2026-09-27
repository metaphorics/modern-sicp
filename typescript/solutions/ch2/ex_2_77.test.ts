// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  magnitude,
  makeComplexFromRealImag,
} from "../../packages/ch2/src/05-generic-operations.js";
import {
  type Complex77,
  magnitudeWithoutSelectors,
  magnitudeWithSelectors,
  makeComplex77,
} from "./ex_2_77.js";

const z: Complex77 = makeComplex77(3, 4);

describe("exercise 2.77: nested apply-generic", () => {
  it("misses without the complex-level selectors, as Louis observed", () => {
    const missed = magnitudeWithoutSelectors(z);
    expect(missed).toEqual({
      _tag: "Error",
      error: { _tag: "NoComplexSelector", op: "magnitude" },
    });
  });

  it("answers 5 once the complex-level selectors are installed", () => {
    expect(magnitudeWithSelectors(z)).toEqual({ _tag: "Ok", value: 5 });
  });

  it("the section system ships the fix: magnitude of z is 5", () => {
    const answered = magnitude(makeComplexFromRealImag(3, 4));
    expect(answered._tag === "Ok" && answered.value === 5).toBe(true);
  });

  it("each dispatch strips exactly one tag, so the trace has two hops", () => {
    // apply-generic (complex) -> complex-level selector -> apply to the
    // inner representation (rectangular) -> representation selector.
    const twoLevel = magnitudeWithSelectors(makeComplex77(0, 5));
    expect(twoLevel).toEqual({ _tag: "Ok", value: 5 });
  });
});
