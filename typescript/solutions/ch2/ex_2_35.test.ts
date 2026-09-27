// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { leaf, node } from "../../packages/ch2/src/02-picture-language.js";
import { countLeavesViaAccumulate } from "./ex_2_35.js";

const x = node(node(leaf(1), leaf(2)), node(leaf(3), leaf(4)));

describe("exercise 2.35", () => {
  it("counts the eight leaves of the doubled tree", () => {
    expect(countLeavesViaAccumulate(node(x, x))).toBe(8);
  });

  it("a single leaf counts one", () => {
    expect(countLeavesViaAccumulate(leaf(9))).toBe(1);
  });
});
