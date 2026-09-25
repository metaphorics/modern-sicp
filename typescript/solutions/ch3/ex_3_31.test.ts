// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { demonstrateWithInitRun, demonstrateWithoutInitRun } from "./ex_3_31.js";

describe("exercise 3.31: why the accepted action runs immediately", () => {
  it("with the initialization run, the probes attach and the sum rises", () => {
    expect(demonstrateWithInitRun()).toEqual([
      { time: 0, name: "sum", value: false },
      { time: 0, name: "carry", value: false },
      { time: 5, name: "sum", value: true },
    ]);
  });

  it("without it, nothing is scheduled from construction and the sum never rises", () => {
    // The probes print nothing on attach (their action is not run),
    // the gates claim nothing from the initial wire values, and the
    // input change alone cannot produce a sum: e stays 0, so the
    // final and-gate answers and 1 and 0 = 0, no change. Log is empty
    // and the book's demo fails.
    expect(demonstrateWithoutInitRun()).toEqual([]);
  });
});
