// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_50, PendingSolution } from "./ex_5_50.js";

describe("exercise 5.50 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_50).toThrow(PendingSolution);
  });
});
