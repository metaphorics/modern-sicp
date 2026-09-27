// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { measureCompoundOr, measurePrimitiveOr, riseA1, riseAndFallA1 } from "./ex_3_29.js";

describe("exercise 3.29: the compound or-gate from and-gates and inverters", () => {
  it("attaches its probe after the settle, at the settle clock", () => {
    // Each measured circuit runs one unprobed settle propagate; the
    // probe then records the settled value at the clock the settle
    // ended on (5: the attach-time items all land by then).
    const compound = measureCompoundOr();
    expect(compound.events).toEqual([{ time: 5, name: "output", value: false }]);
    const primitive = measurePrimitiveOr();
    expect(primitive.events).toEqual([{ time: 5, name: "output", value: false }]);
  });

  it("the compound gate answers a rise 7 later, the primitive 5", () => {
    const compound = riseA1(measureCompoundOr());
    // Rise at clock 5: input inverter (2) + and-gate (3) + output
    // inverter (2) = the change at time 12.
    expect(compound).toEqual([
      { time: 5, name: "output", value: false },
      { time: 12, name: "output", value: true },
    ]);
    const primitive = riseA1(measurePrimitiveOr());
    // Rise at clock 5, one orGateDelay later the same answer.
    expect(primitive).toEqual([
      { time: 5, name: "output", value: false },
      { time: 10, name: "output", value: true },
    ]);
  });

  it("a rise and fall measure the compound delay in both directions", () => {
    const compound = riseAndFallA1(measureCompoundOr());
    expect(compound).toEqual([
      { time: 5, name: "output", value: false },
      { time: 12, name: "output", value: true },
      { time: 19, name: "output", value: false },
    ]);
  });

  it("both gates agree on the settled value before the rise", () => {
    const compound = measureCompoundOr();
    const primitive = measurePrimitiveOr();
    expect(compound.output.getSignal()).toBe(false);
    expect(primitive.output.getSignal()).toBe(false);
  });
});
