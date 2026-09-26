// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_41, PendingSolution } from "./ex_5_41.js";

describe("exercise 5.41 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_41).toThrow(PendingSolution);
  });
});
