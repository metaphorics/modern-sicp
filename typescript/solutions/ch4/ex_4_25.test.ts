// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";

import { armedStrictError, descendingStrictError, ex_4_25, lazyAnswers } from "./ex_4_25.js";

describe("exercise 4.25: unless breaks under applicative order", () => {
  it("the lazy factorial bottoms out at 120", async () => {
    const transcript = await Effect.runPromise(lazyAnswers());
    expect(transcript[transcript.length - 1]).toBe("120");
  });

  it("the armed call dies in the division under strict arguments", async () => {
    const error = await Effect.runPromise(armedStrictError());
    expect(error.message).toBe("/: expects a nonzero divisor list");
  });

  it("the strict factorial only stops at the budget", async () => {
    const error = await Effect.runPromise(descendingStrictError());
    expect(error.message).toContain("still descending");
  });

  it("reports the contrast", () => {
    expect(ex_4_25()).toContain("120");
    expect(ex_4_25()).toContain("still descending");
  });
});
