// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_39, PendingSolution } from "./ex_5_39.js";

describe("exercise 5.39 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_39).toThrow(PendingSolution);
  });
});
