// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamRef, streamTake } from "../../packages/ch3/src/05-streams.js";

import { S } from "./ex_3_56.js";

/** True when repeated division by 2, 3, and 5 exhausts x: no prime
 * factors other than 2, 3, or 5, the statement's membership test. */
const isFiveSmooth = (x: number): boolean => {
  let n = x;
  for (const p of [2, 3, 5]) {
    while (n % p === 0) {
      n /= p;
    }
  }
  return n === 1;
};

describe("exercise 3.56: Hamming's S via merge", () => {
  it("enumerates the 5-smooth numbers in ascending order without repetition", () => {
    expect(streamTake(S, 12)).toEqual([1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16]);
  });

  it("every element of a long prefix has no prime factors other than 2, 3, 5", () => {
    for (const x of streamTake(S, 60)) {
      expect(isFiveSmooth(x)).toBe(true);
    }
  });

  it("a long prefix is strictly increasing, so nothing repeats", () => {
    let prev = streamRef(S, 0);
    for (const x of streamTake(S, 60).slice(1)) {
      expect(x).toBeGreaterThan(prev);
      prev = x;
    }
  });

  it("misses no 5-smooth number: the prefix contains all of them up to its last element", () => {
    const xs = streamTake(S, 60);
    const last = streamRef(S, 59);
    for (let n = 1; n <= last; n += 1) {
      if (isFiveSmooth(n)) {
        expect(xs).toContain(n);
      }
    }
  });

  it("keeps climbing through the Hamming numbers", () => {
    expect(streamRef(S, 49)).toBe(243);
    expect(streamRef(S, 59)).toBe(384);
  });
});
