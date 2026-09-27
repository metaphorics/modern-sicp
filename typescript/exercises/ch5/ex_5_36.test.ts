// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_36, PendingSolution } from "./ex_5_36.js";

describe("exercise 5.36 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_36).toThrow(PendingSolution);
  });
});
