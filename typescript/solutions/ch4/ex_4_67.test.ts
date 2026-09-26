// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { microshaft } from "../../packages/ch4/src/04-logic.js";
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
  it("terminates the book's recursive outranked-by rule with bounded answers", () => {
    const engine = microshaft();
    engine.load(`
      (rule (outranked-by ?staff-person ?boss)
        (or (supervisor ?staff-person ?boss)
            (and (outranked-by ?staff-person ?middle-manager)
                 (supervisor ?middle-manager ?boss))))
    `);

    expect(engine.answers("(outranked-by (Hacker Alyssa P) ?boss)", 20)).toEqual([
      "(outranked-by (Hacker Alyssa P) (Bitdiddle Ben))",
    ]);
  });

  it("keeps history branch-local", () => {
    expect(ex_4_67()).toContain("branch-local");
  });
});
