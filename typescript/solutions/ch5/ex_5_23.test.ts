// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { makeEvaluator } from "../../packages/ch5/src/04-eceval.js";
import { derivedExpressionTranscript } from "./ex_5_23.js";

const valuesOf = (transcript: readonly string[]): string[] => {
  const values: string[] = [];
  for (let i = 0; i < transcript.length; i += 1) {
    if (transcript[i] === ";;; EC-Eval value:") values.push(transcript[i + 1] as string);
  }
  return values;
};

describe("exercise 5.23 derived expressions", () => {
  it("answers cond clauses, a let application, and the edge clauses", () => {
    expect(valuesOf(derivedExpressionTranscript())).toEqual([
      "ok",
      "zero",
      "one",
      "many",
      "6",
      "#t",
      "#f",
    ]);
  });
  it("the base evaluator rejects the same cond, so the transformers carry it", () => {
    expect(() => makeEvaluator("(cond ((= 1 1)))").run()).toThrow();
  });
});
