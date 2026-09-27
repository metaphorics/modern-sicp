// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_24, PendingSolution } from "./ex_5_24.js";

describe("exercise 5.24 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_24).toThrow(PendingSolution);
  });
});
