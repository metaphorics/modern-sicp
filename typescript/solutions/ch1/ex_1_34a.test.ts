// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { fAny, runtimeSelfApplicationError } from "./ex_1_34a.js";

describe("exercise 1.34a", () => {
  it("the erased f still computes on proper arguments", () => {
    expect(fAny((z: number): number => z + 7)).toBe(9);
  });

  it("the erased self-application fails at runtime with a TypeError", () => {
    expect(runtimeSelfApplicationError()).toStrictEqual({
      kind: "TypeError",
      message: "g is not a function",
    });
  });

  it("the message names the parameter that ended up holding 2", () => {
    expect(runtimeSelfApplicationError().message).toContain("g is not a function");
  });
});
