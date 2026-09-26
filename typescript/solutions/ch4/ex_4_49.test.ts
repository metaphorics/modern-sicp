// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_49, sentences } from "./ex_4_49.js";

describe("exercise 4.49: generation", () => {
  it("the first six sentences descend the first alternatives", async () => {
    expect(await Effect.runPromise(sentences(6))).toStrictEqual([
      "(sentence (simple-noun-phrase (article the) (noun student)) (verb studies))",
      "(sentence (simple-noun-phrase (article the) (noun student)) (verb lectures))",
      "(sentence (simple-noun-phrase (article the) (noun student)) (verb eats))",
      "(sentence (simple-noun-phrase (article the) (noun student)) (verb sleeps))",
      "(sentence (simple-noun-phrase (article the) (noun professor)) (verb studies))",
      "(sentence (simple-noun-phrase (article the) (noun professor)) (verb lectures))",
    ]);
  });

  it("reports the boring descent", () => {
    expect(ex_4_49()).toContain("first alternatives");
  });
});
