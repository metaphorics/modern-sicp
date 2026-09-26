// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_62, lastPairAnswers } from "./ex_4_62.js";

describe("exercise 4.62: last-pair", () => {
  it("finds the singleton, final element, and partially specified element", () => {
    const [single, three, partial] = lastPairAnswers();
    expect(single).toStrictEqual(["(last-pair (3) (3))"]);
    expect(three).toStrictEqual(["(last-pair (1 2 3) (3))"]);
    expect(partial).toStrictEqual(["(last-pair (2 3) (3))"]);
  });

  it("identifies the reverse-direction termination caveat", () => {
    expect(ex_4_62()).toContain("not a terminating reverse search");
  });
});
