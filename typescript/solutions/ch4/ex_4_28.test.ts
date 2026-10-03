// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { runForcedOperator, runUnforcedOperator } from "./ex_4_28.js";

describe("exercise 4.28: forcing the operator", () => {
  it("the forced operator answers 5", () => {
    const result = runForcedOperator();
    expect(result.transcript).toEqual(["5"]);
    expect(result.outcome.tag).toBe("ok");
    if (result.outcome.tag === "ok") {
      expect(result.outcome.value).toBeUndefined();
    }
  });

  it("the unforced operator hands the thunk to apply and faults", () => {
    const result = runUnforcedOperator();
    expect(result.outcome.tag).toBe("error");
    if (result.outcome.tag === "error") {
      expect(result.outcome.error.tag).toBe("not-callable");
    }
    expect(result.transcript).toEqual([]);
  });
});
