// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamRef, streamTake } from "../../packages/ch3/src/05-streams.js";

import { type AdditionCounter, fibsCounting, fibsPlainCounting } from "./ex_3_57.js";

const newCounter = (): AdditionCounter => ({ additions: 0 });

describe("exercise 3.57: additions performed computing fibs elements", () => {
  it("the instrumented addStreams fibs is still the Fibonacci stream", () => {
    expect(streamTake(fibsCounting(newCounter()), 10)).toEqual([0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    expect(streamRef(fibsPlainCounting(newCounter()), 10)).toBe(55);
  });

  it("memoized delay: reaching element n costs n - 1 additions", () => {
    const c6 = newCounter();
    streamRef(fibsCounting(c6), 6);
    expect(c6.additions).toBe(5);

    const c10 = newCounter();
    streamRef(fibsCounting(c10), 10);
    expect(c10.additions).toBe(9);
  });

  it("plain-thunk delay: the additions grow exponentially", () => {
    const measured: number[] = [];
    for (let n = 6; n <= 10; n += 1) {
      const c = newCounter();
      streamRef(fibsPlainCounting(c), n);
      measured.push(c.additions);
    }
    expect(measured).toEqual([26, 46, 79, 133, 221]);

    const c6 = newCounter();
    streamRef(fibsPlainCounting(c6), 6);
    const c10 = newCounter();
    streamRef(fibsPlainCounting(c10), 10);
    expect(c10.additions).toBeGreaterThan(8 * c6.additions);
  });
});
