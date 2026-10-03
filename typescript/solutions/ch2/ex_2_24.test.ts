// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { leafCount, listLength, printedForm } from "./ex_2_24.js";

describe("exercise 2.24", () => {
  it("the printed form nests the inner lists", () => {
    expect(printedForm()).toBe("[1, [2, [3, 4]]]");
  });

  it("the outer list has two elements", () => {
    expect(listLength()).toBe(2);
  });

  it("the tree reading has four leaves", () => {
    expect(leafCount()).toBe(4);
  });
});
