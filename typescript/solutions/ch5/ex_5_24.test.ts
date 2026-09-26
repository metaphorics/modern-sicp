// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { basicCondTranscript } from "./ex_5_24.js";

const valuesOf = (transcript: readonly string[]): string[] => {
  const values: string[] = [];
  for (let i = 0; i < transcript.length; i += 1) {
    if (transcript[i] === ";;; EC-Eval value:") values.push(transcript[i + 1] as string);
  }
  return values;
};

describe("exercise 5.24 cond as a basic form", () => {
  it("walks clauses, selects else, and answers the book's edge cases", () => {
    expect(valuesOf(basicCondTranscript())).toEqual([
      "ok",
      "zero",
      "one",
      "many",
      "#t",
      "#f",
      "10",
    ]);
  });
});
