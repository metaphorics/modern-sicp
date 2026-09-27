// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_34, PendingSolution } from "./ex_5_34.js";

describe("exercise 5.34 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_34).toThrow(PendingSolution);
  });
});
