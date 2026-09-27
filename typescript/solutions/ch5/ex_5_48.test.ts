// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_48 } from "./ex_5_48.js";

describe("exercise 5.48", () => {
  it("records a block and runs it on the next assembly", () => {
    const answers = ex_5_48();
    expect(answers[0]).toContain("ok");
    expect(answers[1]).toContain("120");
  });
});
