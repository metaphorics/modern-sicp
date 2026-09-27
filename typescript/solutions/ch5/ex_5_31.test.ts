// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_31 } from "./ex_5_31.js";

describe("exercise 5.31", () => {
  it("keeps the book's save pattern for the four combinations", () => {
    const answers = ex_5_31();
    expect(answers[0]).toBe("(f 'x 'y): ");
    expect(answers[1]).toBe("((f) 'x 'y): ");
    expect(answers[2]).toBe("(f (g 'x) y): (save proc) (save argl) (restore argl) (restore proc)");
    expect(answers[3]).toBe("(f (g 'x) 'y): (save proc) (save argl) (restore argl) (restore proc)");
  });
});
