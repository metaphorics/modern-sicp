// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  collisionAnswers,
  counterAdvances,
  directApplicationSize,
  occursCheckRefuses,
  renamedHeadVars,
  renamingAdvances,
} from "./ex_4_79.js";

describe("exercise 4.79: environments instead of renaming", () => {
  it("answers Irad under the name collision", () => {
    expect(collisionAnswers()).toStrictEqual(['grandson("Irad", "Cain")']);
    expect(directApplicationSize()).toBe(1);
  });

  it("renames rule variables with fresh application ids", () => {
    const vars = renamedHeadVars();
    expect(vars).toHaveLength(2);
    expect(vars.every((name) => name.includes("-"))).toBe(true);
    expect(renamingAdvances()).toBe(true);
    expect(counterAdvances()).toBe(true);
  });

  it("refuses cyclic extension through the occurs check", () => {
    expect(occursCheckRefuses()).toBe(true);
  });
});
