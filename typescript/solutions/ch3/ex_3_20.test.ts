// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makePair, tracePairAliasing } from "./ex_3_20.js";

describe("exercise 3.20", () => {
  it("makePair reads back its slots and mutates them", () => {
    const p = makePair(1, 2);
    expect(p.getX()).toBe(1);
    expect(p.getY()).toBe(2);
    p.setX(17);
    expect(p.getX()).toBe(17);
    expect(p.getY()).toBe(2);
  });

  it("two makePair calls build distinct objects", () => {
    const p = makePair(1, 2);
    const q = makePair(1, 2);
    expect(Object.is(p, q)).toBe(false);
  });

  it("the traced sequence reports the alias and the 17", () => {
    const report = tracePairAliasing();
    expect(report.zCarIsZCdr).toBe(true);
    expect(report.zCdrIsX).toBe(true);
    expect(report.readCarX).toBe(17);
    expect(report.readThroughZ).toBe(17);
  });

  it("the log narrates the four steps in book order", () => {
    const report = tracePairAliasing();
    expect(report.log.map((event) => event._tag)).toEqual([
      "PairCreated",
      "PairCreated",
      "Aliased",
      "SlotSet",
      "ReadThroughAlias",
    ]);
  });

  it("a pre-mutation read pins the before value at 1", () => {
    const x = makePair<number, number>(1, 2);
    const z = makePair(x, x);
    expect(z.getY().getX()).toBe(1);
    z.getY().setX(17);
    expect(x.getX()).toBe(17);
  });
});
