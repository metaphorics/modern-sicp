// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_41 } from "./ex_5_41.js";

describe("exercise 5.41", () => {
  it("finds the lexical addresses", () => {
    expect(ex_5_41()).toEqual(["c: (1 2)", "x: (2 0)", "w: not-found"]);
  });
});
