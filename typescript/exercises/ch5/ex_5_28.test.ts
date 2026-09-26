// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_28, PendingSolution } from "./ex_5_28.js";

describe("exercise 5.28 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_28).toThrow(PendingSolution);
  });
});
