// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_40, PendingSolution } from "./ex_5_40.js";

describe("exercise 5.40 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_40).toThrow(PendingSolution);
  });
});
