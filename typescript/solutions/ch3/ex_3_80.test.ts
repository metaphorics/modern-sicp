// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamRef, streamTake } from "../../packages/ch3/src/05-streams.js";

import { rlc } from "./ex_3_80.js";

describe("exercise 3.80: the series rlc circuit", () => {
  it("starts the state streams from vC0 = 10 and iL0 = 0", () => {
    const { vC, iL } = rlc(1, 1, 0.2, 0.1)(10, 0);
    expect(streamTake(vC, 8)).toEqual([
      10, 10, 9.5, 8.55, 7.220000000000001, 5.5955, 3.77245, 1.8519300000000003,
    ]);
    expect(streamTake(iL, 8)).toEqual([0, 1, 1.9, 2.66, 3.249, 3.6461, 3.84104, 3.834181]);
  });

  it("agrees step for step with a direct Euler integration of the two ODEs", () => {
    const { vC, iL } = rlc(1, 1, 0.2, 0.1)(10, 0);
    const dt = 0.1;
    const R = 1;
    const L = 1;
    const C = 0.2;
    let v = 10;
    let i = 0;
    let maxDiff = Math.abs(streamRef(vC, 0) - v) + Math.abs(streamRef(iL, 0) - i);
    for (let n = 1; n <= 400; n += 1) {
      const dv = -i / C;
      const di = v / L - (R / L) * i;
      v += dt * dv;
      i += dt * di;
      maxDiff = Math.max(maxDiff, Math.abs(streamRef(vC, n) - v), Math.abs(streamRef(iL, n) - i));
    }
    expect(maxDiff).toBe(0);
    expect(streamRef(vC, 400)).toBe(-0.000154630904199953);
    expect(streamRef(iL, 400)).toBe(0.0001260669490826979);
  });
});
