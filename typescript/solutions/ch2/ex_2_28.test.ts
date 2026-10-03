// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { leaf, node, showList } from "../../packages/ch2/src/02-picture-language.js";

import { fringe } from "./ex_2_28.js";

describe("exercise 2.28", () => {
  const x = node(node(leaf(1), leaf(2)), node(leaf(3), leaf(4)));

  it("fringe reads the leaves left to right", () => {
    expect(showList(fringe(x))).toBe("[1, 2, 3, 4]");
  });

  it("the doubled tree lists the leaves twice", () => {
    expect(showList(fringe(node(x, x)))).toBe("[1, 2, 3, 4, 1, 2, 3, 4]");
  });
});
