// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { inverterAnswers, lastOpenEnded, lastPairAnswers } from "./ex_4_62.js";

describe("exercise 4.62: last-pair rules", () => {
  it("answers the three terminating book queries", () => {
    const [singleton, three, constrained] = lastPairAnswers();
    expect(singleton).toStrictEqual(['last-pair(["3"], ["3"])']);
    expect(three).toStrictEqual(['last-pair(["1", "2", "3"], ["3"])']);
    expect(constrained).toStrictEqual(['last-pair(["2", "3"], ["3"])']);
  });

  it("exhibits the open-ended query without running it", () => {
    expect(lastOpenEnded.tag).toBe("atom");
  });

  it("addition 4.62a: the inverter answers both directions", () => {
    const [low, high] = inverterAnswers();
    expect(low).toStrictEqual(['logic-not("0", "1")']);
    expect(high).toStrictEqual(['logic-not("1", "0")']);
  });
});
