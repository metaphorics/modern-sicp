// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

import { Option } from "effect";
import fc from "fast-check";
import { describe, expect, it } from "vitest";

import type { List } from "./list.js";
import { car, cdr, cons, list, nil } from "./list.js";

const toArray = <A>(l: List<A>): Array<A> => {
  const out: Array<A> = [];
  let current: List<A> = l;
  while (current._tag === "Cons") {
    out.push(current.head);
    current = current.tail;
  }
  return out;
};

describe("List", () => {
  it("car and cdr destructure a cons pair", () => {
    const pair = cons(1, cons(2, nil));
    expect(car(pair)).toStrictEqual(Option.some(1));
    expect(cdr(pair)).toStrictEqual(Option.some(cons(2, nil)));
  });

  it("car and cdr of the empty list are nothing, never a throw", () => {
    expect(car(nil)).toStrictEqual(Option.none());
    expect(cdr(nil)).toStrictEqual(Option.none());
  });

  it("round-trips: to-array inverts list", () => {
    fc.assert(
      fc.property(fc.array(fc.integer()), (xs) => {
        expect(toArray(list(...xs))).toEqual(xs);
      }),
    );
  });

  it("cons builds up right-nested structure in order", () => {
    expect(toArray(list("a", "b", "c"))).toEqual(["a", "b", "c"]);
    expect(nil).toStrictEqual({ _tag: "Nil" });
  });
});
