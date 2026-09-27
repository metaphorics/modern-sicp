// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_59, meetingAnswers } from "./ex_4_59.js";

describe("exercise 4.59: meeting-time", () => {
  it("finds Friday division meetings and Alyssa's Wednesday meetings", () => {
    const [friday, wednesday] = meetingAnswers();
    expect(friday).toStrictEqual(["(meeting administration (Friday 1pm))"]);
    expect(wednesday).toStrictEqual([
      "(meeting-time (Hacker Alyssa P) (Wednesday 4pm))",
      "(meeting-time (Hacker Alyssa P) (Wednesday 3pm))",
    ]);
  });

  it("states both meeting results", () => {
    expect(ex_4_59()).toContain("Wednesday 4pm");
  });
});
