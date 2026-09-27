// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_23, PendingSolution } from "./ex_5_23.js";

describe("exercise 5.23 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_23).toThrow(PendingSolution);
  });
});
