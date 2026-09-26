// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_51, PendingSolution } from "./ex_5_51.js";

describe("exercise 5.51 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_51).toThrow(PendingSolution);
  });
});
