// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";

import { answers, ex_4_30 } from "./ex_4_30.js";

describe("exercise 4.30: forcing in eval-sequence", () => {
  it("pins the book's sessions under both sequence rules", async () => {
    const observed = await Effect.runPromise(answers());
    expect(observed.forEachText).toBe("\n57\n321\n88|done");
    expect(observed.forEachCy).toBe("\n57\n321\n88|done");
    expect(observed.p1Text).toBe("(1 2)");
    expect(observed.p1Cy).toBe("(1 2)");
    expect(observed.p2Text).toBe("1");
    expect(observed.p2Cy).toBe("(1 2)");
  });

  it("answers part d by keeping the text's rule", () => {
    expect(ex_4_30()).toContain("keeps the text's rule");
  });
});
