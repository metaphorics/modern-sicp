// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { flattenOrders } from "./ex_4_73.js";

const hacker = '["Hacker", "Alyssa", "P"]';
const louis = '["Reasoner", "Louis"]';

describe("exercise 4.73: why flattenStream delays", () => {
  it("interleaves the second stream instead of burying it", () => {
    const [flattened, appended] = flattenOrders();
    expect(flattened).toStrictEqual([
      hacker,
      louis,
      '["Fect", "Cy", "D"]',
      '["Tweakit", "Lem", "E"]',
    ]);
    expect(appended).toStrictEqual([
      hacker,
      '["Fect", "Cy", "D"]',
      '["Tweakit", "Lem", "E"]',
      louis,
    ]);
  });
});
