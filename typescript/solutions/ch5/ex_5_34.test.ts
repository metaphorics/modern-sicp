// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_34 } from "./ex_5_34.ts";

describe("exercise 5.34 iterative factorial compilation", () => {
  it("pairs executed saves across call branches and compiles tail calls to gotos", () => {
    const lines = ex_5_34();
    expect(lines[1]).toBe("every executed save is paired: true");
    expect(lines[2]).toBe("tail calls compile to gotos");
  });
});
