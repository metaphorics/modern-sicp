// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_35, PendingSolution } from "./ex_5_35.js";

describe("exercise 5.35 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_35).toThrow(PendingSolution);
  });
});
