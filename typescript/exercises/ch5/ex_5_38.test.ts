// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_38, PendingSolution } from "./ex_5_38.js";

describe("exercise 5.38 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_38).toThrow(PendingSolution);
  });
});
