// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { num } from "../../packages/ch4/src/syntax/ast.js";
import {
  analyzeSequenceAlyssa,
  analyzeSequenceText,
  makeCounters,
  runSequence,
  type SequenceCounters,
} from "./ex_4_23.js";

describe("exercise 4.23: analyze-sequence comparison", () => {
  it("a one-expression body executed five times: 5 leaves and no walk", () => {
    const text = runSequence(
      (counters: SequenceCounters) => analyzeSequenceText([num(1)], counters),
      5,
    );
    expect(text).toEqual({ leafRuns: 5, walks: 0 });
    const alyssa = runSequence(
      (counters: SequenceCounters) => analyzeSequenceAlyssa([num(1)], counters),
      5,
    );
    expect(alyssa).toEqual({ leafRuns: 5, walks: 5 });
  });

  it("a two-expression body executed five times: 10 leaves and one walk per run", () => {
    const text = runSequence(
      (counters: SequenceCounters) => analyzeSequenceText([num(1), num(2)], counters),
      5,
    );
    expect(text).toEqual({ leafRuns: 10, walks: 0 });
    const alyssa = runSequence(
      (counters: SequenceCounters) => analyzeSequenceAlyssa([num(1), num(2)], counters),
      5,
    );
    expect(alyssa).toEqual({ leafRuns: 10, walks: 5 });
  });

  it("an empty sequence fails in both versions", () => {
    const env = new Session("core").globalEnv();
    const text = analyzeSequenceText([], makeCounters())(env);
    expect(text.tag).toBe("error");
    const alyssa = analyzeSequenceAlyssa([], makeCounters())(env);
    expect(alyssa.tag).toBe("error");
  });
});
