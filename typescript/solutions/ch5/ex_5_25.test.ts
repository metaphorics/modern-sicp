// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  ex_5_25,
  lazyFactorialValue,
  lazyMemoizationValues,
  lazyUnusedArgumentValue,
} from "./ex_5_25.js";

describe("exercise 5.25 normal-order evaluation", () => {
  it("answers the chapter 1.5 divergence test with 0, where strict order runs forever", () => {
    expect(ex_5_25()).toBe("0");
  });
  it("returns 42 although the unused argument would fault if evaluated", () => {
    expect(lazyUnusedArgumentValue()).toBe("42");
  });
  it("forces a thunked argument once: both references see it and count moved one step", () => {
    expect(lazyMemoizationValues().slice(-2)).toEqual(["(1 . 1)", "1"]);
  });
  it("still computes the applicative factorial", () => {
    expect(lazyFactorialValue()).toBe("120");
  });
});
