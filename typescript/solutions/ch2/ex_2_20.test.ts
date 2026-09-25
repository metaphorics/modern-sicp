// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { showList } from "../../packages/ch2/src/02-picture-language.js";

import { sameParity } from "./ex_2_20.js";

describe("exercise 2.20", () => {
  it("keeps the odd arguments when the first is odd", () => {
    expect(showList(sameParity(1, 2, 3, 4, 5, 6, 7))).toBe("(1 3 5 7)");
  });

  it("keeps the even arguments when the first is even", () => {
    expect(showList(sameParity(2, 3, 4, 5, 6, 7))).toBe("(2 4 6)");
  });

  it("a lone first argument is its own parity class", () => {
    expect(showList(sameParity(7))).toBe("(7)");
  });
});
