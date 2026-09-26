// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_31, PendingSolution } from "./ex_5_31.js";

describe("exercise 5.31 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_31).toThrow(PendingSolution);
  });
});
