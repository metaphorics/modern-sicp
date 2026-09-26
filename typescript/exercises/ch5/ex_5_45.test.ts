// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_45, PendingSolution } from "./ex_5_45.js";

describe("exercise 5.45 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_45).toThrow(PendingSolution);
  });
});
