// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { allOutranked, whoOutranksBen } from "./ex_4_64.js";
import { LoopDetector } from "./ex_4_67.js";

describe("exercise 4.67: a loop detector", () => {
  it("refuses a pattern already under derivation", () => {
    const detector = new LoopDetector();
    expect(detector.enter(whoOutranksBen)).toBe(true);
    expect(detector.enter(whoOutranksBen)).toBe(false);
    expect(detector.history()).toHaveLength(1);
  });

  it("releases patterns on completion and admits distinct ones", () => {
    const detector = new LoopDetector();
    detector.enter(whoOutranksBen);
    detector.leave(whoOutranksBen);
    expect(detector.enter(whoOutranksBen)).toBe(true);
    expect(detector.enter(allOutranked)).toBe(true);
    expect(detector.history()).toHaveLength(2);
  });
});
