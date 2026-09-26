// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_67, repeatsActiveCall } from "./ex_4_67.js";

describe("exercise 4.67: active-call loop detection", () => {
  it("rejects the same instantiated subproblem on the current path", () => {
    const active = { pattern: "(supervisor Ada ?boss)", bindings: { "?boss": "Ben" } };
    expect(
      repeatsActiveCall({ pattern: "(supervisor Ada ?boss)", bindings: { "?boss": "Ben" } }, [
        active,
      ]),
    ).toBe(true);
  });

  it("allows a changed frame or a completed call on another branch", () => {
    const active = { pattern: "(supervisor Ada ?boss)", bindings: { "?boss": "Ben" } };
    expect(
      repeatsActiveCall({ pattern: "(supervisor Ada ?boss)", bindings: { "?boss": "Oliver" } }, [
        active,
      ]),
    ).toBe(false);
    expect(
      repeatsActiveCall({ pattern: "(job Ada ?job)", bindings: { "?boss": "Ben" } }, [active]),
    ).toBe(false);
  });

  it("keeps history branch-local", () => {
    expect(ex_4_67()).toContain("branch-local");
  });
});
