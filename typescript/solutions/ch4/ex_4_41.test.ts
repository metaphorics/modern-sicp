// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { solutions as searchSolutions } from "./ex_4_39.js";
import { solveDwelling } from "./ex_4_41.js";

describe("exercise 4.41: an ordinary program", () => {
  it("the enumeration answers exactly one solution, the search program's", () => {
    const found = solveDwelling();
    expect(found).toHaveLength(1);
    expect(found[0]).toEqual({ baker: 3, cooper: 2, fletcher: 4, miller: 5, smith: 1 });
    expect(searchSolutions("book")).toEqual([
      "{ baker: 3, cooper: 2, fletcher: 4, miller: 5, smith: 1 }",
    ]);
  });
});
