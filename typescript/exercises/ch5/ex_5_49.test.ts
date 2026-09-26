// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_49, PendingSolution } from "./ex_5_49.js";

describe("exercise 5.49 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_49).toThrow(PendingSolution);
  });
});
