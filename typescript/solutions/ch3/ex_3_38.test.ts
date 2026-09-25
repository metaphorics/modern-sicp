// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { runInterleaving } from "../../packages/ch3/src/04-concurrency.js";

import {
  interleavedValues,
  interleavingOnlyValues,
  type JointAccount,
  makeJointAccount,
  maryProcess,
  paulProcess,
  peterProcess,
  sequentialValues,
} from "./ex_3_38.js";

describe("exercise 3.38: interleaved balance outcomes", () => {
  it("the six sequential orders produce 45, 35, 45, 50, 40, and 40", () => {
    expect(sequentialValues()).toEqual([45, 35, 45, 50, 40, 40]);
  });

  it("the distinct sequential values are 35, 40, 45, and 50", () => {
    expect([...new Set(sequentialValues())].sort((a, b) => a - b)).toEqual([35, 40, 45, 50]);
  });

  it("the one ordering of Figure 3.29's shape ends at 110", () => {
    // Mary and Peter both access 100, Mary writes 50, Peter writes 110
    // last from the stale access.
    const account: JointAccount = makeJointAccount();
    const peter = peterProcess(account);
    const paul = paulProcess(account);
    const mary = maryProcess(account);
    runInterleaving([peter, paul, mary], [0, 2, 0, 2, 2, 0]);
    expect(account.balance).toBe(110);
  });

  it("the interleaving-only values are the run values outside the sequential six", () => {
    const only = interleavingOnlyValues();
    // Last-writer-wins values: a single stale write lands last.
    expect(only).toContain(110);
    expect(only).toContain(80);
    // Chained staleness: a second wave reads a raced value, like
    // Mary's two accesses straddling Peter's write producing 45 for
    // Paul to spend down to 25.
    expect(only).toContain(55);
    expect(only).toContain(60);
    expect(only).toContain(90);
    expect(only).toContain(25);
    for (const value of only) {
      expect(value % 5).toBe(0);
      expect(value).toBeGreaterThan(0);
    }
    expect(only).toEqual([25, 30, 55, 60, 65, 70, 80, 90, 110]);
  });

  it("the full interleaved set is pinned by the runs", () => {
    expect(interleavedValues()).toEqual([25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 80, 90, 110]);
  });
});
