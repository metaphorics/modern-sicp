// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_30, PendingSolution } from "./ex_5_30.js";

describe("exercise 5.30 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_30).toThrow(PendingSolution);
  });
});
