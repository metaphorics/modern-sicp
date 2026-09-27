// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamRef, streamTake } from "../../packages/ch3/src/05-streams.js";

import { solve2nd } from "./ex_3_78.js";

describe("exercise 3.78: solve-2nd, the second-order feedback loop", () => {
  it("starts the damped oscillator from y0 = 1 and dy0 = 0", () => {
    const y = solve2nd(-0.1, -1, 1, 0, 0.01);
    expect(streamTake(y, 5)).toEqual([1, 1, 0.9999, 0.9997001, 0.9994004099]);
  });

  it("decays the damped envelope: |y at t = 40| below |y at t = 5|", () => {
    const y = solve2nd(-0.1, -1, 1, 0, 0.01);
    expect(streamRef(y, 500)).toBe(0.18510721623125634);
    expect(streamRef(y, 4000)).toBe(-0.09990880783812048);
    expect(Math.abs(streamRef(y, 4000))).toBeLessThan(Math.abs(streamRef(y, 500)));
  });

  it("decays the windowed envelope maxima the same way", () => {
    const y = solve2nd(-0.1, -1, 1, 0, 0.01);
    const peak = (from: number, to: number): number => {
      let m = 0;
      for (let n = from; n <= to; n += 1) {
        m = Math.max(m, Math.abs(streamRef(y, n)));
      }
      return m;
    };
    expect(peak(460, 500)).toBe(0.18510721623125634);
    expect(peak(3960, 4000)).toBe(0.09990880783812048);
  });

  it("tracks sin(t) for the undamped case at step 0.01", () => {
    const y = solve2nd(0, -1, 0, 1, 0.01);
    const y1 = streamRef(y, 100);
    const y2 = streamRef(y, 200);
    const y3 = streamRef(y, 300);
    expect(y1).toBe(0.8456705645316805);
    expect(y2).toBe(0.918463576916705);
    expect(y3).toBe(0.14335314503859142);
    expect(Math.abs(y1 - Math.sin(1))).toBe(0.0041995797237840415);
    expect(Math.abs(y2 - Math.sin(2))).toBe(0.009166150091023284);
    expect(Math.abs(y3 - Math.sin(3))).toBe(0.0022331369787242095);
    expect(Math.abs(y1 - Math.sin(1))).toBeLessThan(1e-2);
    expect(Math.abs(y2 - Math.sin(2))).toBeLessThan(1e-2);
    expect(Math.abs(y3 - Math.sin(3))).toBeLessThan(1e-2);
  });

  it("brings the undamped tracking under 1e-3 at step 0.001", () => {
    const y = solve2nd(0, -1, 0, 1, 0.001);
    expect(Math.abs(streamRef(y, 1000) - Math.sin(1))).toBe(0.00042066029255583004);
    expect(Math.abs(streamRef(y, 2000) - Math.sin(2))).toBe(0.0009100294804466058);
    expect(Math.abs(streamRef(y, 3000) - Math.sin(3))).toBe(0.00021283022344251168);
    expect(Math.abs(streamRef(y, 1000) - Math.sin(1))).toBeLessThan(1e-3);
    expect(Math.abs(streamRef(y, 2000) - Math.sin(2))).toBeLessThan(1e-3);
    expect(Math.abs(streamRef(y, 3000) - Math.sin(3))).toBeLessThan(1e-3);
  });
});
