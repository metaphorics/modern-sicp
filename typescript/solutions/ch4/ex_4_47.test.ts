// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { format } from "../../packages/ch4/src/read.js";
import { louisPrefix } from "./ex_4_47.js";

const firstParse =
  '["sentence", ["noun-phrase", ["article", "the"], ["noun", "cat"]], ["verb", "eats"]]';

describe("exercise 4.47: Louis's parse-verb-phrase", () => {
  it("gets a finite answer prefix and bounds the unending continuation", () => {
    const run = louisPrefix(2, 16);
    expect(run.answers.map((value) => format(value))).toStrictEqual([firstParse]);
    expect(run.status).toBe("cut-off");
    expect(run.failures).toBeGreaterThan(0);
    expect(run.steps).toBe(16);
  });
});
