// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { showMList } from "../../packages/ch3/src/03-mutable-data.js";
import { makeZ1, makeZ2, setToWow } from "./ex_3_15.js";

describe("exercise 3.15: setToWow shows sharing", () => {
  it("z1 shares one [a, b] pair between car and cdr", () => {
    const z1 = makeZ1();
    expect(showMList(z1)).toBe("[[a, b], a, b]");
    expect(Object.is(z1.head, z1.tail)).toBe(true);
  });

  it("z2 holds two distinct (a b) copies and prints the same shape", () => {
    const z2 = makeZ2();
    expect(showMList(z2)).toBe("[[a, b], a, b]");
    expect(Object.is(z2.head, z2.tail)).toBe(false);
  });

  it("one setToWow on z1 shows up through both pointers", () => {
    expect(showMList(setToWow(makeZ1()))).toBe("[[wow, b], wow, b]");
  });

  it("one setToWow on z2 shows up through the car only", () => {
    expect(showMList(setToWow(makeZ2()))).toBe("[[wow, b], a, b]");
  });
});
