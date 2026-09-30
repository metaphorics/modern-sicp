// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { answers } from "./ex_4_32.js";

describe("exercise 4.32: streams versus lazy lists", () => {
  it("car forces the head only: 7 with one mark", () => {
    const observed = answers();
    expect(observed.car).toEqual(["7", "1"]);
  });

  it("cdr forces only the armed slot on demand: the division with one mark", () => {
    const observed = answers();
    expect(observed.cdr).toEqual(["Infinity", "1"]);
  });

  it("the self-referential ones constructs with nothing forced", () => {
    const observed = answers();
    expect(observed.ones).toEqual(["2", "0"]);
  });

  it("the strict contrast evaluates both slots at construction", () => {
    const observed = answers();
    expect(observed.strict).toEqual(["Infinity", "2"]);
  });
});
