// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_39, lexicalOperations } from "./ex_5_39.ts";

describe("exercise 5.39 lexical-address lookup", () => {
  it("runs the session whose free variable is read by address", () => {
    expect(ex_5_39().some((line) => line.includes("120") || line.includes("121"))).toBe(true);
  });
  it("the operations walk frames and positions", () => {
    expect(Object.keys(lexicalOperations()).sort()).toEqual([
      "lexicalAddressLookup",
      "lexicalAddressSet",
    ]);
  });
});
