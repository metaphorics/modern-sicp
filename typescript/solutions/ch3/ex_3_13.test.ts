// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { isMNil, type MCons, type MList, mlist } from "../../packages/ch3/src/03-mutable-data.js";
import { cycleDemo, makeCycle } from "./ex_3_13.js";

const pairOf = <A>(l: MList<A>): MCons<A> => {
  if (isMNil(l)) {
    throw new Error("expected a pair");
  }
  return l;
};

describe("exercise 3.13: make-cycle builds a ring", () => {
  it("the third pair tails back to the first pair", () => {
    const z = makeCycle(mlist("a", "b", "c"));
    const second = pairOf(z.tail);
    const third = pairOf(second.tail);
    expect(z.head).toBe("a");
    expect(second.head).toBe("b");
    expect(third.head).toBe("c");
    expect(Object.is(third.tail, z)).toBe(true);
  });

  it("walking three tails from the first pair lands on it again", () => {
    const z = makeCycle(mlist("a", "b", "c"));
    const one = pairOf(z.tail);
    const two = pairOf(one.tail);
    const three = pairOf(two.tail);
    expect(Object.is(three, z)).toBe(true);
  });

  it("makeCycle mutates its argument and returns it", () => {
    const base = mlist("a", "b", "c");
    const z = makeCycle(base);
    expect(Object.is(z, base)).toBe(true);
  });

  it("the demo pins the ring by identity, never printing it", () => {
    expect(cycleDemo()).toEqual({
      firstHead: "a",
      secondHead: "b",
      thirdHead: "c",
      thirdTailsBackToFirst: true,
    });
  });
});
