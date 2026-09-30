// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { decodedSample, printDecoded } from "./ex_2_67.js";

describe("exercise 2.67", () => {
  it("decodes the 13-bit sample message to (A D A B B C A)", () => {
    expect(printDecoded(decodedSample)).toBe("[A, D, A, B, B, C, A]");
  });
});
