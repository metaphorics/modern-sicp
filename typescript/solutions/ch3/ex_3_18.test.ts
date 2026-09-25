// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  type MCons,
  type MList,
  mcons,
  mlist,
  mnil,
} from "../../packages/ch3/src/03-mutable-data.js";

import { hasCycle } from "./ex_3_18.js";

/** The book's ring: the last tail of `l` is set to `backTo`, the
 * set-cdr! that exercise 3.13 used. */
const ringWithLastTail = <A>(l: MList<A>, backTo: MList<A>): MCons<A> => {
  if (l._tag === "MNil") {
    throw new Error("ring needs a first pair");
  }
  let last: MCons<A> = l;
  let rest: MList<A> = l;
  while (rest._tag === "MCons") {
    last = rest;
    rest = rest.tail;
  }
  last.tail = backTo;
  return l;
};

/** Walks to the last pair of a nonempty list, the walk 3.12 names
 * `last-pair`, narrowed from the empty end. */
const lastPairOf = <A>(l: MList<A>): MCons<A> => {
  if (l._tag === "MNil") {
    throw new Error("lastPairOf needs a first pair");
  }
  let last: MCons<A> = l;
  let rest: MList<A> = l;
  while (rest._tag === "MCons") {
    last = rest;
    rest = rest.tail;
  }
  return last;
};

describe("exercise 3.18", () => {
  it("a plain list has no cycle", () => {
    expect(hasCycle(mlist(1, 2, 3))).toBe(false);
  });

  it("the empty list has no cycle", () => {
    expect(hasCycle(mlist())).toBe(false);
  });

  it("a two-pair ring is a cycle", () => {
    const empty: MList<number> = { _tag: "MNil" };
    const a = mcons(1, empty);
    const b = mcons(2, a);
    a.tail = b;
    expect(hasCycle(b)).toBe(true);
  });

  it("a list whose last pair loops back into itself is a cycle", () => {
    // The 3.13 self-loop: the last pair's tail is the pair itself.
    const l = mlist("x", "y", "z");
    const looped = ringWithLastTail(l, l);
    expect(hasCycle(looped)).toBe(true);
    const selfLoop = mlist("x", "y", "z");
    lastPairOf(selfLoop).tail = lastPairOf(selfLoop);
    expect(hasCycle(selfLoop)).toBe(true);
  });

  it("two lists sharing a suffix are both acyclic", () => {
    const shared = mlist(3, 4, 5);
    const longer = mcons(1, mcons(2, shared));
    const shorter = mcons(0, shared);
    expect(hasCycle(longer)).toBe(false);
    expect(hasCycle(shorter)).toBe(false);
  });
});
