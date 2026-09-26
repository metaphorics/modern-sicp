// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_46, trace } from "./ex_4_46.js";

describe("exercise 4.46: left-to-right operands", () => {
  it("enumerates left-major and the left operand announces first", async () => {
    const observed = await Effect.runPromise(trace(3));
    expect(observed.answers).toStrictEqual(["(1 1)", "(1 2)", "(2 1)"]);
    expect(observed.displayed).toBe("leftrightright");
  });

  it("replays the right operand when the left choice resumes", async () => {
    const observed = await Effect.runPromise(trace(4));
    expect(observed.answers).toStrictEqual(["(1 1)", "(1 2)", "(2 1)", "(2 2)"]);
    expect(observed.displayed).toBe("leftrightright");
  });

  it("reports the order", () => {
    expect(ex_4_46()).toContain("left to right");
  });
});
