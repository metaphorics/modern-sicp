// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import type { List } from "../../packages/ch2/src/02-picture-language.js";
import { length, showList } from "../../packages/ch2/src/02-picture-language.js";
import { queens } from "./ex_2_42.js";
import { queensSwapped } from "./ex_2_43.js";

/** Flattens a list into an array. */
const toArray = <A>(xs: List<A>): A[] => {
  const out: A[] = [];
  for (let rest = xs; rest._tag === "Cons"; rest = rest.tail) {
    out.push(rest.head);
  }
  return out;
};

const standard6 = queens(6);
const swapped6 = queensSwapped(6);

describe("exercise 2.43", () => {
  it("Louis's interchanged order still finds the 4 six-by-six solutions", () => {
    expect(length(swapped6)).toBe(4);
  });

  it("finds the same solution set as the standard order", () => {
    expect(toArray(swapped6).map(showList).sort()).toEqual(toArray(standard6).map(showList).sort());
  });
});
