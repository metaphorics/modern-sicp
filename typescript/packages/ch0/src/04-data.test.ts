// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: Chapter 0 section 0.4

import { describe, expect, it } from "vitest";

import {
  area,
  car,
  cdr,
  cons,
  equalFrac,
  equalList,
  list,
  nil,
  type Point,
  some,
} from "./04-data.js";

describe("data", () => {
  it("a tuple is the anonymous pair", () => {
    const p: Point = [3, 4];
    const [x, y] = p;
    expect(x).toBe(3);
    expect(y).toBe(4);
  });

  it("records compare part by part, never by identity", () => {
    expect(equalFrac({ num: 1, den: 2 }, { num: 1, den: 2 })).toBe(true);
    const { num, den } = { num: 1, den: 2 };
    expect(num).toBe(1);
    expect(den).toBe(2);
    const f = { num: 1, den: 2 };
    const twin = { num: 1, den: 2 };
    expect(f === twin).toBe(false);
  });

  it("area dispatches on the tag of the union", () => {
    expect(area({ _tag: "Circle", r: 1 })).toBe(Math.PI);
    expect(area({ _tag: "Rect", w: 2, h: 3 })).toBe(6);
  });

  it("car and cdr destructure a cons pair", () => {
    const pair = cons(1, cons(2, nil));
    expect(car(pair)).toEqual(some(1));
    expect(cdr(pair)).toEqual(some(cons(2, nil)));
  });

  it("car and cdr of the empty list are nothing, never a throw", () => {
    expect(car(nil)).toEqual({ _tag: "None" });
    expect(cdr(nil)).toEqual({ _tag: "None" });
  });

  it("list builds up right-nested structure in order", () => {
    const xs = list("a", "b", "c");
    expect(equalList(xs, cons("a", cons("b", cons("c", nil))), (a, b) => a === b)).toBe(true);
    expect(equalList(xs, list("a", "b"), (a, b) => a === b)).toBe(false);
    expect(nil).toEqual({ _tag: "Nil" });
  });
});
