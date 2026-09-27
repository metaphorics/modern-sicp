// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { Stream, singletonStream } from "../../packages/ch4/src/04-logic.js";
import { simpleStreamFlatmap } from "./ex_4_74.js";

const from = (a: number[], i = 0): Stream<number> => {
  const head = a[i];
  return head === undefined ? Stream.empty() : Stream.cons(head, () => from(a, i + 1));
};
describe("4.74", () =>
  it("skips empty streams and retains singleton order", () =>
    expect(
      simpleStreamFlatmap(
        (n: number) => (n % 2 ? singletonStream(n * 10) : Stream.empty()),
        from([1, 2, 3, 4]),
      ).take(3),
    ).toEqual([10, 30])));
