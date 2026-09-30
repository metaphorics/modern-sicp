// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  append,
  cons,
  type List,
  list,
  type Showable,
  showList,
} from "../../packages/ch2/src/02-picture-language.js";

import { appendResult, consResult, listResult } from "./ex_2_26.js";

describe("exercise 2.26", () => {
  const x: List<number> = list(1, 2, 3);
  const y: List<number> = list(4, 5, 6);

  it("append concatenates into one six-element list", () => {
    expect(showList(append(x, y))).toBe("[1, 2, 3, 4, 5, 6]");
    expect(appendResult()).toBe("[1, 2, 3, 4, 5, 6]");
  });

  it("cons makes x the first element over y's spine", () => {
    expect(showList(cons<Showable>(x, y))).toBe("[[1, 2, 3], 4, 5, 6]");
    expect(consResult()).toBe("[[1, 2, 3], 4, 5, 6]");
  });

  it("list makes a two-element list of the lists", () => {
    expect(showList(list<Showable>(x, y))).toBe("[[1, 2, 3], [4, 5, 6]]");
    expect(listResult()).toBe("[[1, 2, 3], [4, 5, 6]]");
  });
});
