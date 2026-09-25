// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  isMNil,
  type MCons,
  type MList,
  mlist,
  mnil,
  showMList,
} from "../../packages/ch3/src/03-mutable-data.js";
import { mystery } from "./ex_3_14.js";

const pairOf = <A>(l: MList<A>): MCons<A> => {
  if (isMNil(l)) {
    throw new Error("expected a pair");
  }
  return l;
};

describe("exercise 3.14: mystery reverses the pointers in place", () => {
  it("returns (d c b a) and consumes v to (a)", () => {
    const v = mlist("a", "b", "c", "d");
    const w = mystery(v);
    expect(showMList(w)).toBe("(d c b a)");
    // The caller's binding kept its first pair, whose tail the loop
    // rewrote to the empty list: v now prints as the one-element list.
    expect(showMList(v)).toBe("(a)");
  });

  it("the reversed chain is the input's own pairs, pointed backwards", () => {
    const v = mlist("a", "b", "c", "d");
    const first = pairOf(v);
    const w = mystery(v);
    // w's front holds d: the original last pair is the new first.
    expect(pairOf(w).head).toBe("d");
    let walks = 0;
    for (let rest = w; rest._tag === "MCons"; rest = rest.tail) {
      walks += 1;
    }
    expect(walks).toBe(4);
    // The original first pair survived as w's last pair, tail empty.
    let last = pairOf(w);
    while (last.tail._tag === "MCons") {
      last = last.tail;
    }
    expect(Object.is(last, first)).toBe(true);
    expect(last.tail).toEqual({ _tag: "MNil" });
  });

  it("the empty list reverses to the empty list", () => {
    expect(showMList(mystery(mnil))).toBe("()");
  });
});
