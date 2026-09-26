// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_50, sampledSentences, seed } from "./ex_4_50.js";

describe("exercise 4.50: ramb", () => {
  it("two evaluators from one seed ramble identically, another seed differs", async () => {
    const left = await Effect.runPromise(sampledSentences(seed, 3));
    const right = await Effect.runPromise(sampledSentences(seed, 3));
    expect(left).toStrictEqual(right);
    expect(left[0]).toBe(
      "(sentence (simple-noun-phrase (article a) (noun professor)) (verb studies))",
    );
    const other = await Effect.runPromise(sampledSentences(7, 3));
    expect(other).not.toStrictEqual(left);
    expect(other[0]).toBe(
      "(sentence (simple-noun-phrase (article a) (noun student)) (verb studies))",
    );
  });

  it("reports the shuffle", () => {
    expect(ex_4_50()).toContain("Fisher-Yates");
  });
});
