// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { qlist, qnum, qsym } from "../../packages/ch2/src/03-symbolic-data.js";
import { equalQ } from "./ex_2_54.js";

describe("exercise 2.54", () => {
  it("agrees with the book's two examples", () => {
    const a = qlist(qsym("this"), qsym("is"), qsym("a"), qsym("list"));
    expect(equalQ(a, qlist(qsym("this"), qsym("is"), qsym("a"), qsym("list")))).toBe(true);
    expect(equalQ(a, qlist(qsym("this"), qlist(qsym("is"), qsym("a")), qsym("list")))).toBe(false);
  });

  it("compares numbers numerically and recursion depth structurally", () => {
    expect(equalQ(qnum(2), qnum(2))).toBe(true);
    expect(equalQ(qnum(2), qnum(3))).toBe(false);
    expect(equalQ(qnum(2), qsym("2"))).toBe(false);
    expect(
      equalQ(qlist(qlist(qnum(1), qnum(2)), qnum(3)), qlist(qlist(qnum(1), qnum(2)), qnum(3))),
    ).toBe(true);
    expect(equalQ(qlist(), qlist())).toBe(true);
    expect(equalQ(qlist(), qlist(qnum(0)))).toBe(false);
  });
});
