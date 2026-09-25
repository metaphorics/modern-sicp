// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { type MCons, type MList, mcons, mlist } from "../../packages/ch3/src/03-mutable-data.js";

import { hasCycleConstantSpace } from "./ex_3_19.js";

/** The book's ring: the last pair's tail set back to the first pair. */
const ringFromSelf = <A>(l: MList<A>): MList<A> => {
  if (l._tag === "MNil") {
    throw new Error("ring needs a first pair");
  }
  let last: MCons<A> = l;
  let rest: MList<A> = l;
  while (rest._tag === "MCons") {
    last = rest;
    rest = rest.tail;
  }
  last.tail = l;
  return l;
};

/** The book's long tail into a two-pair ring: a run of k pairs whose
 * last tail points at the ring's first pair. */
const tailedRing = <A>(run: MList<A>, ring: MList<A>): MList<A> => {
  if (run._tag === "MNil") {
    throw new Error("tail run needs a first pair");
  }
  let last: MCons<A> = run;
  let rest: MList<A> = run;
  while (rest._tag === "MCons") {
    last = rest;
    rest = rest.tail;
  }
  last.tail = ring;
  return run;
};

describe("exercise 3.19", () => {
  it("a plain list has no cycle, hare reaches the end", () => {
    expect(hasCycleConstantSpace(mlist(1, 2, 3))).toBe(false);
  });

  it("the empty list has no cycle", () => {
    expect(hasCycleConstantSpace(mlist())).toBe(false);
  });

  it("a two-pair ring is a cycle", () => {
    const empty2: MList<number> = { _tag: "MNil" };
    const a = mcons(1, empty2);
    const b = mcons(2, a);
    a.tail = b;
    expect(hasCycleConstantSpace(b)).toBe(true);
  });

  it("a list whose last pair loops back to its own head is a cycle", () => {
    expect(hasCycleConstantSpace(ringFromSelf(mlist("x", "y", "z")))).toBe(true);
  });

  it("a long run into a two-pair ring is still caught", () => {
    const empty: MList<number> = { _tag: "MNil" };
    const a = mcons(100, empty);
    const b = mcons(101, a);
    a.tail = b;
    const run = mlist(1, 2, 3, 4, 5, 6, 7, 8);
    expect(hasCycleConstantSpace(tailedRing(run, b))).toBe(true);
  });

  it("two lists sharing a suffix are both acyclic", () => {
    const shared = mlist(3, 4, 5);
    const longer = mcons(1, mcons(2, shared));
    const shorter = mcons(0, shared);
    expect(hasCycleConstantSpace(longer)).toBe(false);
    expect(hasCycleConstantSpace(shorter)).toBe(false);
  });
});
