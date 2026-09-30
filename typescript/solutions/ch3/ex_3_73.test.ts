// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { consStream, ones, type Stream, streamTake } from "../../packages/ch3/src/05-streams.js";

import { rc1 } from "./ex_3_73.js";

describe("exercise 3.73: the RC circuit as a signal processor", () => {
  it("charging from a constant current, rc1 answers the first eight voltages", () => {
    expect(streamTake(rc1(ones, 0), 8)).toEqual([
      0, 0.5, 0.95, 1.355, 1.7195, 2.04755, 2.342795, 2.6085155,
    ]);
  });

  it("tracks the closed form v(t) = R i (1 - e^(-t/RC)) within the step error", () => {
    const voltages = streamTake(rc1(ones, 0), 8);
    const R = 5;
    const C = 1;
    const dt = 0.5;
    const drive = 1;
    voltages.forEach((v, k) => {
      const t = k * dt;
      const exact = R * drive * (1 - Math.exp(-t / (R * C)));
      expect(Math.abs(v - exact)).toBeLessThan(0.15);
    });
  });

  it("when the current stops, the capacitor discharges back toward zero", () => {
    const zeros: Stream<number> = consStream(0, () => zeros);
    const pulse = (n: number): Stream<number> =>
      n <= 0 ? zeros : consStream(1, () => pulse(n - 1));
    const voltage = streamTake(rc1(pulse(10), 0), 31);
    const charging = voltage.slice(0, 11);
    const discharging = voltage.slice(10);
    expect(charging.every((v, i) => i === 0 || v > (charging[i - 1] ?? 0))).toBe(true);
    expect(voltage[10]).toBe(3.2566077995000002);
    expect(
      discharging.every((v, i) => i === 0 || v < (discharging[i - 1] ?? Number.POSITIVE_INFINITY)),
    ).toBe(true);
    expect(voltage[30]).toBe(0.39592748157676544);
    expect(voltage[30]).toBeLessThan(0.5);
  });
});
