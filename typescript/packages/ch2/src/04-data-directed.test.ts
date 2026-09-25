// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 2.4

import { describe, expect, it } from "vitest";

import {
  addComplex,
  addComplexTagged,
  alyssaPolar,
  angle,
  angleExplicit,
  applyGeneric,
  applyGenericMessage,
  benRectangular,
  type ComplexPair,
  divComplex,
  imagPart,
  imagPartExplicit,
  isPolar,
  isRectangular,
  magnitude,
  magnitudeExplicit,
  makeFromMagAng,
  makeFromMagAngPolar,
  makeFromRealImag,
  makeFromRealImagMessage,
  makeFromRealImagRectangular,
  mulComplex,
  realPart,
  realPartExplicit,
  showApplyGenericError,
  subComplex,
  type Tagged,
  type TaggedComplex,
  valueOr,
} from "./04-data-directed.js";

const quarterPi = Math.PI / 4;
const quarterAngle = 0.9272952180016122; // atan2(4, 3), the angle of (3, 4)

describe("section 2.4: multiple representations for abstract data", () => {
  it("2.4.1 selects Ben's rectangular pairs", () => {
    const z = benRectangular.makeFromRealImag(3, 4);
    expect(benRectangular.realPart(z)).toBe(3);
    expect(benRectangular.imagPart(z)).toBe(4);
    expect(benRectangular.magnitude(z)).toBe(5);
    expect(benRectangular.angle(z)).toBe(quarterAngle);
  });

  it("2.4.1 selects Alyssa's polar pairs", () => {
    const z = alyssaPolar.makeFromMagAng(5, quarterAngle);
    expect(alyssaPolar.magnitude(z)).toBe(5);
    expect(alyssaPolar.angle(z)).toBe(quarterAngle);
    const up = alyssaPolar.makeFromMagAng(2, Math.PI / 2);
    expect(alyssaPolar.angle(up)).toBe(1.5707963267948966);
  });

  it("2.4.1 reproduces z from either constructor, as the prose requires", () => {
    for (const arith of [benRectangular, alyssaPolar]) {
      const z = arith.makeFromRealImag(3, 4);
      const fromParts = arith.makeFromRealImag(arith.realPart(z), arith.imagPart(z));
      const fromPolar = arith.makeFromMagAng(arith.magnitude(z), arith.angle(z));
      for (const w of [fromParts, fromPolar]) {
        expect(arith.realPart(w)).toBeCloseTo(arith.realPart(z), 12);
        expect(arith.imagPart(w)).toBeCloseTo(arith.imagPart(z), 12);
      }
    }
  });

  it("2.4.1 adds and subtracts in rectangular form", () => {
    const arith = benRectangular;
    const z1 = arith.makeFromRealImag(1, 2);
    const z2 = arith.makeFromRealImag(3, 4);
    expect(arith.realPart(addComplex(arith, z1, z2))).toBe(4);
    expect(arith.imagPart(addComplex(arith, z1, z2))).toBe(6);
    expect(arith.realPart(subComplex(arith, z2, z1))).toBe(2);
    expect(arith.imagPart(subComplex(arith, z2, z1))).toBe(2);
  });

  it("2.4.1 multiplies and divides in polar form", () => {
    const arith = alyssaPolar;
    const p1 = arith.makeFromMagAng(2, quarterPi);
    const p2 = arith.makeFromMagAng(3, quarterPi);
    expect(arith.magnitude(mulComplex(arith, p1, p2))).toBe(6);
    expect(arith.angle(mulComplex(arith, p1, p2))).toBe(1.5707963267948966);
    expect(arith.magnitude(divComplex(arith, mulComplex(arith, p1, p2), p2))).toBe(2);
    expect(arith.angle(divComplex(arith, mulComplex(arith, p1, p2), p2))).toBe(quarterPi);
  });

  it("2.4.2 tags datums and reads the tags back", () => {
    expect(isRectangular(makeFromRealImagRectangular(3, 4))).toBe(true);
    expect(isPolar(makeFromMagAngPolar(5, quarterAngle))).toBe(true);
    expect(makeFromRealImagRectangular(3, 4)._tag).toBe("rectangular");
    const tagged: Tagged<"rectangular", ComplexPair> = makeFromRealImagRectangular(3, 4);
    expect(tagged.contents).toEqual([3, 4]);
  });

  it("2.4.2 dispatches explicitly on the tag", () => {
    expect(realPartExplicit(makeFromRealImagRectangular(3, 4))).toBe(3);
    expect(magnitudeExplicit(makeFromMagAngPolar(5, quarterAngle))).toBe(5);
  });

  it("2.4.2 keeps the same arithmetic over tagged datums", () => {
    const z1 = makeFromRealImagRectangular(1, 2);
    const z2 = makeFromRealImagRectangular(3, 4);
    const sum: TaggedComplex = addComplexTagged(z1, z2);
    expect(realPartExplicit(sum)).toBe(4);
    expect(imagPartExplicit(sum)).toBe(6);
    expect(angleExplicit(makeFromMagAngPolar(2, Math.PI / 2))).toBe(1.5707963267948966);
  });

  it("2.4.3 answers the generic selectors through the table", () => {
    const z = makeFromRealImag(3, 4);
    expect(z._tag).toBe("Ok");
    expect(valueOr(realPart(valueOr(z, makeFromRealImagRectangular(0, 0))), 0)).toBe(3);
    expect(valueOr(imagPart(valueOr(z, makeFromRealImagRectangular(0, 0))), 0)).toBe(4);
    const polar = makeFromMagAng(2, Math.PI / 2);
    expect(valueOr(magnitude(valueOr(polar, makeFromMagAngPolar(0, 0))), 0)).toBe(2);
    expect(valueOr(angle(valueOr(polar, makeFromMagAngPolar(0, 0))), 0)).toBe(1.5707963267948966);
  });

  it("2.4.3 reports why a lookup failed", () => {
    const unknownShape: Tagged<string, ComplexPair> = { _tag: "oblique", contents: [3, 4] };
    const missed = applyGeneric("real-part", unknownShape);
    expect(missed._tag).toBe("Error");
    if (missed._tag === "Error") {
      expect(showApplyGenericError(missed.error)).toBe(
        "No method for these types: APPLY-GENERIC (real-part (oblique))",
      );
    }
  });

  it("2.4.3 message passing answers the object directly", () => {
    const z = makeFromRealImagMessage(3, 4);
    expect(valueOr(applyGenericMessage(z, "real-part"), 0)).toBe(3);
    expect(valueOr(applyGenericMessage(z, "imag-part"), 0)).toBe(4);
    expect(valueOr(applyGenericMessage(z, "magnitude"), 0)).toBe(5);
    expect(valueOr(applyGenericMessage(z, "angle"), 0)).toBe(quarterAngle);
  });
});
