// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { runInterleaving } from "../../packages/ch3/src/04-concurrency.js";

import {
  cubeProcess,
  serializedOutcomes,
  squareProcess,
  unsynchronizedOutcomes,
} from "./ex_3_40.js";

describe("exercise 3.40: all values of concurrent x squared and cubed", () => {
  it("the unsynchronized square and cube land on 100 through 1000000, by powers of ten", () => {
    expect(unsynchronizedOutcomes()).toEqual([100, 1000, 10000, 100000, 1000000]);
  });

  it("the extreme 100 is the square finishing last from a full-stale access pair", () => {
    // The square's two accesses both saw 10 before the cube wrote
    // 1000, and the square's stale 100 lands last: both square
    // accesses, the whole cube, then the square's write.
    const cell = { x: 10 };
    runInterleaving([squareProcess(cell), cubeProcess(cell)], [0, 0, 1, 1, 1, 1, 0]);
    expect(cell.x).toBe(100);
  });

  it("the serialized pair always leaves one million", () => {
    expect(serializedOutcomes()).toEqual([1000000]);
  });
});
