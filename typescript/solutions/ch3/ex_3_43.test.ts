// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  firstMultisetViolatingFinal,
  firstVersionSums,
  noAccountSerializationFinals,
  noAccountSerializationSums,
  serializedFinals,
} from "./ex_3_43.js";

describe("exercise 3.43: exchange preserves the multiset of balances", () => {
  it("the first version breaks the multiset: 10, 20, and 30 become 10, 10, and 40", () => {
    expect(firstMultisetViolatingFinal()).toEqual([40, 10, 10]);
  });

  it("even so, the first version never loses money: every schedule sums to 60", () => {
    expect(firstVersionSums()).toEqual([60]);
  });

  it("the serialized exchange keeps the balances a permutation of the start", () => {
    const finals = serializedFinals();
    expect(finals).toEqual([
      [30, 10, 20],
      [20, 30, 10],
    ]);
  });

  it("without per-account serialization the races destroy money outright", () => {
    // Sums strictly below 60 are vanished dollars; none exceed 60,
    // since no schedule can invent money either.
    expect(noAccountSerializationSums()).toEqual([40, 50, 60]);
    const sum60 = noAccountSerializationFinals().filter(([a1, a2, a3]) => a1 + a2 + a3 === 60);
    expect(sum60.length).toBeGreaterThan(0);
  });
});
