// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamRef } from "../../packages/ch3/src/05-streams.js";

import {
  improveCallsFor,
  improveCallsForSameStream,
  sqrt2ConvergedGuess,
  sqrtStreamExternal,
  sqrtStreamLocal,
  sqrtStreamUnmemoizedExternal,
  sqrtStreamUnmemoizedLocal,
} from "./ex_3_63.js";

describe("exercise 3.63: why sqrtStream localizes its guesses", () => {
  it("computes each guess once with the memoized local binding", () => {
    expect(improveCallsFor(sqrtStreamLocal, 2, 5)).toEqual({
      guess: sqrt2ConvergedGuess,
      calls: 5,
    });
    expect(improveCallsFor(sqrtStreamLocal, 2, 8)).toEqual({
      guess: sqrt2ConvergedGuess,
      calls: 8,
    });
  });

  it("recomputes each level from scratch in Louis's external version", () => {
    expect(improveCallsFor(sqrtStreamExternal, 2, 5)).toEqual({
      guess: sqrt2ConvergedGuess,
      calls: 15,
    });
    expect(improveCallsFor(sqrtStreamExternal, 2, 8)).toEqual({
      guess: sqrt2ConvergedGuess,
      calls: 36,
    });
  });

  it("extends a shared memoized stream by only the new guesses", () => {
    expect(improveCallsForSameStream(sqrtStreamLocal, 2, 5, 8)).toEqual({
      guessM: sqrt2ConvergedGuess,
      guessN: sqrt2ConvergedGuess,
      callsM: 5,
      callsN: 8,
    });
    expect(improveCallsForSameStream(sqrtStreamExternal, 2, 5, 8)).toEqual({
      guessM: sqrt2ConvergedGuess,
      guessN: sqrt2ConvergedGuess,
      callsM: 15,
      callsN: 36,
    });
  });

  it("loses the advantage entirely with plain-lambda delay: the local shape recomputes too", () => {
    expect(improveCallsFor(sqrtStreamUnmemoizedLocal, 2, 5)).toEqual({
      guess: sqrt2ConvergedGuess,
      calls: 15,
    });
    expect(improveCallsFor(sqrtStreamUnmemoizedLocal, 2, 8)).toEqual({
      guess: sqrt2ConvergedGuess,
      calls: 36,
    });
    expect(improveCallsForSameStream(sqrtStreamUnmemoizedLocal, 2, 5, 8)).toEqual({
      guessM: sqrt2ConvergedGuess,
      guessN: sqrt2ConvergedGuess,
      callsM: 15,
      callsN: 51,
    });
  });

  it("answers the second question: without memoization the two versions count the same", () => {
    expect(improveCallsFor(sqrtStreamUnmemoizedExternal, 2, 5)).toEqual(
      improveCallsFor(sqrtStreamUnmemoizedLocal, 2, 5),
    );
    expect(improveCallsFor(sqrtStreamUnmemoizedExternal, 2, 8)).toEqual(
      improveCallsFor(sqrtStreamUnmemoizedLocal, 2, 8),
    );
    expect(improveCallsForSameStream(sqrtStreamUnmemoizedExternal, 2, 5, 8)).toEqual(
      improveCallsForSameStream(sqrtStreamUnmemoizedLocal, 2, 5, 8),
    );
  });

  it("converges to the same double in every variant", () => {
    expect(
      streamRef(
        sqrtStreamLocal(2, (g, x) => (g + x / g) / 2),
        8,
      ),
    ).toBe(sqrt2ConvergedGuess);
    expect(
      streamRef(
        sqrtStreamExternal(2, (g, x) => (g + x / g) / 2),
        8,
      ),
    ).toBe(sqrt2ConvergedGuess);
  });
});
