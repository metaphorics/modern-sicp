// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { reverseAnswers, reverseBackward } from "./ex_4_68.js";

describe("exercise 4.68: reverse through append-to-form", () => {
  it("reverses (1 2 3) forward", () => {
    const [forward] = reverseAnswers();
    expect(forward).toStrictEqual(['reverse(["1", "2", "3"], ["3", "2", "1"])']);
  });

  it("exhibits the backward query without running it", () => {
    expect(reverseBackward.tag).toBe("atom");
  });

  it("addition 4.68a: palindromes reverse to themselves", () => {
    const [, yes, no] = reverseAnswers();
    expect(yes).toStrictEqual(['palindrome(["1", "2", "1"])']);
    expect(no).toStrictEqual(["No."]);
  });
});
