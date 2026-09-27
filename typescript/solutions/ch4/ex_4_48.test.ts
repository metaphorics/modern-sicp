// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_48, parses } from "./ex_4_48.js";

describe("exercise 4.48: extending the grammar", () => {
  it("one adjective attaches before the noun", async () => {
    expect(await Effect.runPromise(parses("the sleepy cat eats"))).toStrictEqual([
      "(sentence (simple-noun-phrase (article the) ((adjective sleepy) (noun cat))) (verb eats))",
    ]);
  });

  it("two adjectives attach in input order", async () => {
    expect(await Effect.runPromise(parses("the quick brown cat sleeps"))).toStrictEqual([
      "(sentence (simple-noun-phrase (article the) ((adjective quick) (adjective brown) (noun cat))) (verb sleeps))",
    ]);
  });

  it("reports the extension", () => {
    expect(ex_4_48()).toContain("two-adjective modifier list");
  });
});
