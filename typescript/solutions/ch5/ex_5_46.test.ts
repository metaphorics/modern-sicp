// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_46 } from "./ex_5_46.ts";

describe("exercise 5.46", () => {
  it("measures fib at three sizes on three machines", () => {
    const rows = ex_5_46();
    expect(rows).toHaveLength(3);
    expect(rows[2]).toContain("n = 7");
  });
});
