// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_29, PendingSolution } from "./ex_5_29.js";

describe("exercise 5.29 scaffold", () => {
  it("is pending", () => {
    expect(ex_5_29).toThrow(PendingSolution);
  });
});
