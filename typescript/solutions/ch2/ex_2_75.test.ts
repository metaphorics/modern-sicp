// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { applyGenericMessage } from "../../packages/ch2/src/04-data-directed.js";
import { makeFromMagAngMessage } from "./ex_2_75.js";

const angleOf34 = Math.atan2(4, 3);

describe("exercise 2.75: message-passing make-from-mag-ang", () => {
  it("answers magnitude and angle from its captured parts", () => {
    const z = makeFromMagAngMessage(5, angleOf34);
    expect(applyGenericMessage(z, "magnitude")).toEqual({ _tag: "Ok", value: 5 });
    expect(applyGenericMessage(z, "angle")).toEqual({
      _tag: "Ok",
      value: 0.9272952180016122,
    });
  });

  it("derives the rectangular parts by the trigonometric identities", () => {
    const z = makeFromMagAngMessage(5, angleOf34);
    const realPart = applyGenericMessage(z, "real-part");
    const imagPart = applyGenericMessage(z, "imag-part");
    expect(realPart._tag === "Ok" ? realPart.value : 0).toBeCloseTo(3, 12);
    expect(imagPart._tag === "Ok" ? imagPart.value : 0).toBeCloseTo(4, 12);
  });

  it("reports the book's Unknown op error for an unanswered message", () => {
    const missed = makeFromMagAngMessage(5, angleOf34)("spin");
    expect(missed).toEqual({
      _tag: "Error",
      error: { _tag: "UnknownMessage", op: "spin", source: "makeFromMagAng" },
    });
  });
});
