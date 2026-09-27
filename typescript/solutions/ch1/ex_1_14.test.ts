// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { countChangeWithNodes } from "./ex_1_14.js";

describe("exercise 1.14", () => {
  it("changing 11 cents counts 4 ways", () => {
    expect(countChangeWithNodes(11).ways).toBe(4);
  });

  it("the tree for 11 cents has 55 nodes and depth 16", () => {
    const { nodes, maxDepth } = countChangeWithNodes(11);
    expect(nodes).toBe(55);
    expect(maxDepth).toBe(16);
  });

  it("the node count grows polynomially and the depth linearly", () => {
    const hundred = countChangeWithNodes(100);
    expect(hundred.nodes).toBe(15499);
    expect(hundred.maxDepth).toBe(105);
    const twoHundred = countChangeWithNodes(200);
    expect(twoHundred.nodes).toBe(229589);
    expect(twoHundred.maxDepth).toBe(205);
  });
});
