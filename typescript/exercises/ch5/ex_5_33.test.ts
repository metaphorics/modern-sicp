// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_33, PendingSolution } from "./ex_5_33.js";

describe("exercise 5.33 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_33).toThrow(PendingSolution);
  });
});
