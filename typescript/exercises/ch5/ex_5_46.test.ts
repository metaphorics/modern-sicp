// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_46, PendingSolution } from "./ex_5_46.js";

describe("exercise 5.46 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_46).toThrow(PendingSolution);
  });
});
