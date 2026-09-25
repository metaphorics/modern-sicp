// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { carOfDoubleQuote, valueOfDoubleQuote } from "./ex_2_55.js";

describe("exercise 2.55", () => {
  it("the car of the hand-built double quote is the symbol quote", () => {
    expect(valueOfDoubleQuote._tag).toBe("Lst");
    expect(carOfDoubleQuote()).toBe("quote");
  });
});
