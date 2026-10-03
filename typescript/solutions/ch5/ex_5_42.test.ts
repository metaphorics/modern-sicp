// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_42 } from "./ex_5_42.ts";

describe("exercise 5.42 lexical addressing in code generators", () => {
  it("bound names compile to addresses and free names to lookups", () => {
    const lines = ex_5_42();
    expect(lines[0]).toContain("lexical address (2, 0)");
    expect(lines[1]).toContain("lexical address (0, 0)");
    expect(lines[3]).toContain("lexical address (1, 0)");
  });
});
