// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { duplicateLabelOutcome } from "./ex_5_08.js";

describe("exercise 5.8 duplicate label detection", () => {
  it("refuses a doubly defined label at assembly time", () => {
    expect(duplicateLabelOutcome()).toBe("the label here is used twice");
  });
});
