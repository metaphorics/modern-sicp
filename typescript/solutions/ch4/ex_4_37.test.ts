// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { format } from "../../packages/ch4/src/read.js";
import { answers, failureCounts, ordinaryAnswers } from "./ex_4_37.js";

const triples = [
  "[3, 4, 5]",
  "[5, 12, 13]",
  "[6, 8, 10]",
  "[8, 15, 17]",
  "[9, 12, 15]",
  "[12, 16, 20]",
];

describe("exercise 4.37: Ben's generator", () => {
  it("Ben's prune answers the same triples", () => {
    const ben = answers();
    const ordinary = ordinaryAnswers();
    expect(ben.answers.map((value) => format(value))).toStrictEqual(triples);
    expect(ordinary.answers.map((value) => format(value))).toStrictEqual(triples);
  });

  it("measures fewer deferred backtracks for the pruned generator", () => {
    const [ordinaryFailures, benFailures] = failureCounts();
    expect(ordinaryFailures).toBeGreaterThan(benFailures);
  });
});
