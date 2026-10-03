// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_43, scanOutDefines } from "./ex_5_43.ts";

describe("exercise 5.43 scan out internal definitions", () => {
  it("turns internal definitions into one declaration block plus assignments", () => {
    expect(ex_5_43()).toEqual(["var-decl", "var-decl", "expr-stmt", "expr-stmt"]);
  });
  it("leaves a body without definitions unchanged", () => {
    expect(scanOutDefines([])).toEqual([]);
  });
});
