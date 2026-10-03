// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { meetingAnswers } from "./ex_4_59.js";

describe("exercise 4.59: meeting time", () => {
  it("finds the Friday meeting by its day", () => {
    const [friday] = meetingAnswers();
    expect(friday).toStrictEqual(['meeting("administration", ["Friday", "1pm"])']);
  });

  it("joins the whole-company meeting with the divisional one on Wednesday", () => {
    const [, wednesday] = meetingAnswers();
    expect(wednesday).toStrictEqual([
      'meeting-time(["Hacker", "Alyssa", "P"], ["Wednesday", "4pm"])',
      'meeting-time(["Hacker", "Alyssa", "P"], ["Wednesday", "3pm"])',
    ]);
  });

  it("addition 4.59a: the two-clause division rule agrees on the divisional leg", () => {
    const [, , division] = meetingAnswers();
    expect(division).toStrictEqual([
      'division-meeting-time(["Hacker", "Alyssa", "P"], ["Wednesday", "3pm"])',
    ]);
  });
});
