// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { uniqueEngine } from "./ex_4_75.js";

describe("4.75", () => {
  it("succeeds for exactly one wizard", () =>
    expect(uniqueEngine().answers("(unique (job ?x (computer wizard)))")).toEqual([
      "(unique (job Ben (computer wizard)))",
    ]));
  it("fails for multiple or zero results", () => {
    const e = uniqueEngine();
    expect(e.answers("(unique (job ?x (computer programmer)))")).toEqual([]);
    expect(e.answers("(unique (job ?x (computer nonexistent)))")).toEqual([]);
  });
});
