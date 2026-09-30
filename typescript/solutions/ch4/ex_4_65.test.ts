// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { distinctWheels, wheelAnswers } from "./ex_4_65.js";

const warbucks = 'wheel(["Warbucks", "Oliver"])';
const ben = 'wheel(["Bitdiddle", "Ben"])';

describe("exercise 4.65: the wheel listed four times", () => {
  it("derives one line per middle path, not per wheel", () => {
    const answers = wheelAnswers();
    expect(answers).toHaveLength(5);
    expect(answers.filter((line) => line === warbucks)).toHaveLength(4);
    expect(answers.filter((line) => line === ben)).toHaveLength(1);
  });

  it("dedupes to the two wheels in first-appearance order", () => {
    expect(distinctWheels()).toStrictEqual([ben, warbucks]);
  });
});
