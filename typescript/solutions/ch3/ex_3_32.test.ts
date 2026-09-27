// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { addToAgenda } from "../../packages/ch3/src/03-mutable-data.js";

import { addToAgendaLifo, demonstrateLifo, runSchedule } from "./ex_3_32.js";

describe("exercise 3.32: the agenda segment is FIFO", () => {
  it("the module agenda runs same-time actions in insertion order", () => {
    expect(runSchedule(addToAgenda)).toEqual(["D", "B", "A", "C"]);
  });

  it("a LIFO segment runs the same-time actions in reverse insertion order", () => {
    expect(runSchedule(addToAgendaLifo)).toEqual(["D", "B", "C", "A"]);
  });

  it("the half-adder under a LIFO agenda changes the sum's timing", () => {
    // Under the FIFO module agenda the same demo pins sum@5, then
    // carry@11 and sum@16 (see 03-mutable-data.test.ts and 3.31).
    // Under LIFO, the e := 1 initialization response lands at the
    // FRONT of the time-5 segment, so it fires before the or-gate's
    // d := 1, the and-gate's same-segment response reads d still 0,
    // and the sum waits for the d-triggered item instead.
    expect(demonstrateLifo()).toEqual([
      { time: 0, name: "sum", value: false },
      { time: 0, name: "carry", value: false },
      { time: 8, name: "sum", value: true },
      { time: 11, name: "carry", value: true },
      { time: 16, name: "sum", value: false },
    ]);
  });
});
