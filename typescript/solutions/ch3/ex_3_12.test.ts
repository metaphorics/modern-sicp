// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { mcons, mlist, mnil, showMList } from "../../packages/ch3/src/03-mutable-data.js";
import { append, appendBang, appendDemo, lastPair } from "./ex_3_12.js";

describe("exercise 3.12: append copies, append! splices", () => {
  it("append builds a fresh chain and leaves x unchanged", () => {
    const x = mlist("a", "b");
    const y = mlist("c", "d");
    const z = append(x, y);
    expect(showMList(z)).toBe("(a b c d)");
    expect(showMList(x)).toBe("(a b)");
    expect(showMList(y)).toBe("(c d)");
  });

  it("lastPair returns the final pair by identity", () => {
    const x = mcons("a", mcons("b", mnil)); // the book's (list 'a 'b)
    expect(Object.is(lastPair(x), x.tail)).toBe(true);
  });

  it("append! splices y onto x and rewrites the final pair of x", () => {
    const x = mlist("a", "b");
    const y = mlist("c", "d");
    const finalPair = lastPair(x);
    const w = appendBang(x, y);
    expect(Object.is(w, x)).toBe(true);
    expect(showMList(w)).toBe("(a b c d)");
    expect(showMList(x)).toBe("(a b c d)");
    expect(Object.is(finalPair.tail, y)).toBe(true);
  });

  it("append! throws on an empty first list", () => {
    expect(() => appendBang(mnil, mlist("c", "d"))).toThrow("append!: empty first list");
  });

  it("the demo pins the book's interaction", () => {
    expect(appendDemo()).toEqual({
      zAfterAppend: "(a b c d)",
      cdrXAfterAppend: "(b)",
      wAfterAppendBang: "(a b c d)",
      cdrXAfterAppendBang: "(b c d)",
      xTailIsY: true,
    });
  });
});
