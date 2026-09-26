// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_52, PendingSolution } from "./ex_5_52.js";

describe("exercise 5.52 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_52).toThrow(PendingSolution);
  });
});
