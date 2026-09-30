// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ignoredAnswers, requiredAnswers } from "./ex_4_54.js";

describe("exercise 4.54: require as a special form", () => {
  it("kills branches only through the require node", () => {
    expect(requiredAnswers()).toStrictEqual(["3", "4"]);
    expect(ignoredAnswers()).toStrictEqual(["1", "2", "3", "4"]);
  });
});
