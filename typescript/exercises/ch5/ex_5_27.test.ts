// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_27, PendingSolution } from "./ex_5_27.js";

describe("exercise 5.27 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_27).toThrow(PendingSolution);
  });
});
