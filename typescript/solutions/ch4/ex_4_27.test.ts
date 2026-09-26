// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";

import { answers, ex_4_27 } from "./ex_4_27.js";

describe("exercise 4.27: lazy identity with set!", () => {
  it("pins the book's sequence: 1, 10, 2, then an unchanging re-display", async () => {
    const observed = await Effect.runPromise(answers());
    expect(observed).toStrictEqual(["ok", "ok", "ok", "1", "10", "2", "10", "2"]);
  });

  it("explains the memoized re-forcing", () => {
    expect(ex_4_27()).toContain("memoizes");
  });
});
