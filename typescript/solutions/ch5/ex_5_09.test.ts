// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { labelOperandOutcome, labelOperandUnderBaseAssembler } from "./ex_5_09.js";

describe("exercise 5.9 labels as operation operands", () => {
  it("the strict assembler refuses a label used as an operation operand", () => {
    expect(labelOperandOutcome()).toBe(
      "an operation input is written (reg r) or (const c), not (label b)",
    );
  });
  it("the base assembler computes the label's instruction address", () => {
    expect(labelOperandUnderBaseAssembler()).toBe(5);
  });
});
