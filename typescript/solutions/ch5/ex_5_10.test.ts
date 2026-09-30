// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_10, gcdInNewSyntax, isLabel, isMove, translate } from "./ex_5_10.ts";

describe("exercise 5.10 new machine syntax", () => {
  it("the translated machine answers gcd", () => {
    expect(ex_5_10(28, 16)).toBe(4);
    expect(ex_5_10(17, 5)).toBe(1);
  });
  it("the syntax procedures recognize their own forms only", () => {
    expect(isLabel(gcdInNewSyntax[0] ?? { tag: "new-move", to: "a", from: "b" })).toBe(true);
    expect(isMove(gcdInNewSyntax[4] ?? { tag: "new-label", name: "x" })).toBe(true);
  });
  it("every new form lowers to typed machine statements", () => {
    for (const line of gcdInNewSyntax) {
      const lowered = translate(line);
      expect(lowered.length).toBeGreaterThan(0);
      for (const statement of lowered) {
        expect(statement.tag).toBeTruthy();
      }
    }
  });
});
