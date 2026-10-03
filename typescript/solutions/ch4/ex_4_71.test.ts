// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { sonOrders, supervisorBranchOrders } from "./ex_4_71.js";

describe("exercise 4.71: why simple-query and disjoin delay", () => {
  it("interleaves rule hits among assertion hits when delayed", () => {
    const [delayed] = sonOrders();
    expect(delayed).toStrictEqual([
      'son("Adam", "Cain")',
      'son("Lamech", "Jabal")',
      'son("Cain", "Enoch")',
      'son("Lamech", "Jubal")',
      'son("Enoch", "Irad")',
      'son("Irad", "Mehujael")',
      'son("Mehujael", "Methushael")',
      'son("Methushael", "Lamech")',
      'son("Ada", "Jabal")',
      'son("Ada", "Jubal")',
    ]);
  });

  it("appends all assertions before rules without the delay", () => {
    const [, appended] = sonOrders();
    expect(appended).toHaveLength(10);
    expect(appended[7]).toBe('son("Ada", "Jubal")');
    expect(appended[8]).toBe('son("Lamech", "Jabal")');
    expect(appended[9]).toBe('son("Lamech", "Jubal")');
  });

  it("interleaves disjoin branches when delayed, appends otherwise", () => {
    const [delayed, appended] = supervisorBranchOrders();
    expect(delayed).toHaveLength(4);
    expect(appended).toHaveLength(4);
    expect(delayed[1]).toContain('["Reasoner", "Louis"]');
    expect(appended[3]).toContain('["Reasoner", "Louis"]');
  });
});
