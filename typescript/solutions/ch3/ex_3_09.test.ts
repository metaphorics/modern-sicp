// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  factorialIterTraced,
  factorialTraced,
  iterativeFrameStates,
  recursiveFrames,
  renderFactorialStructures,
} from "./ex_3_09.js";

describe("exercise 3.9: the two factorials' frame structures", () => {
  it("the recursive version reaches one frame per pending n: depth 6 at factorial(6)", () => {
    expect(factorialTraced(6)).toEqual({ result: 720, deepest: 6 });
    expect(factorialTraced(1)).toEqual({ result: 1, deepest: 1 });
    expect(factorialTraced(4)).toEqual({ result: 24, deepest: 4 });
  });

  it("the iterative version never leaves its one frame", () => {
    expect(factorialIterTraced(6)).toEqual({ result: 720, deepest: 1 });
    expect(factorialIterTraced(1)).toEqual({ result: 1, deepest: 1 });
    expect(factorialIterTraced(4)).toEqual({ result: 24, deepest: 1 });
  });

  it("at the deepest point six recursive frames are live, each pointing at global", () => {
    expect(recursiveFrames(6).map((frame) => frame.n)).toEqual([6, 5, 4, 3, 2, 1]);
    expect(recursiveFrames(6)[0]).toEqual({ n: 6, pending: "6 * factorial(5)" });
    expect(recursiveFrames(6)[5]).toEqual({ n: 1, pending: "1" });
  });

  it("the iterative frame is reassigned, never stacked", () => {
    const states = iterativeFrameStates(6);
    expect(states).toHaveLength(6);
    expect(states[0]).toEqual({ product: 1, counter: 1 });
    expect(states[5]).toEqual({ product: 120, counter: 6 });
  });

  it("the rendered structures match the instrumented run", () => {
    expect(renderFactorialStructures(6)).toBe(
      [
        "factorial(6) recursive: 6 frames live at the deepest point",
        "factorialIter(6) iterative: 1 frame reused across 6 iterations",
        "",
        "recursive (all live)          iterative (one frame, reused)",
        "n: 6   -> global              product: 1, counter: 1 -> global",
        "n: 5   -> global              product: 1, counter: 2 -> global",
        "n: 4   -> global              product: 2, counter: 3 -> global",
        "n: 3   -> global              product: 6, counter: 4 -> global",
        "n: 2   -> global              product: 24, counter: 5 -> global",
        "n: 1   -> global              product: 120, counter: 6 -> global",
        "",
        "each frame waits for the one   the frame is reassigned, never",
        "below it, then multiplies      stacked: nothing accumulates",
      ].join("\n"),
    );
  });
});
