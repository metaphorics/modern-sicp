// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list } from "../../packages/ch2/src/02-picture-language.js";

import { forEach } from "./ex_2_23.js";

describe("exercise 2.23", () => {
  it("applies f to each element left to right and returns true", () => {
    const seen: number[] = [];
    expect(forEach((x: number) => seen.push(x), list(57, 321, 88))).toBe(true);
    expect(seen).toEqual([57, 321, 88]);
  });

  it("the empty list applies nothing and still returns true", () => {
    const seen: number[] = [];
    expect(forEach((x: number) => seen.push(x), list())).toBe(true);
    expect(seen).toEqual([]);
  });
});
