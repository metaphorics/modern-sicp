// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  naiveWheelCount,
  programmerSalaries,
  programmerSalarySum,
  salvagedWheelCount,
} from "./ex_4_66.js";

describe("exercise 4.66: accumulation over frames", () => {
  it("sums the programmers' salaries from matched frames", () => {
    expect(programmerSalaries()).toStrictEqual([40000, 35000]);
    expect(programmerSalarySum()).toBe(75000);
  });

  it("counts five derivations but two wheels", () => {
    expect(naiveWheelCount()).toBe(5);
    expect(salvagedWheelCount()).toBe(2);
  });
});
