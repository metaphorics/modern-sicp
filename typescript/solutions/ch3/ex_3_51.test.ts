// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamRef } from "../../packages/ch3/src/05-streams.js";

import { ex351, makeUnmemoizedX, makeX } from "./ex_3_51.js";

describe("exercise 3.51: show reveals memoized delay timing", () => {
  it("prints the canonical transcript: 0 at definition, 0 through 5, then 6 and 7", () => {
    const run = ex351();
    expect(run.afterDefine).toEqual(["0"]);
    expect(run.afterRef5).toEqual(["0", "1", "2", "3", "4", "5"]);
    expect(run.afterRef7).toEqual(["0", "1", "2", "3", "4", "5", "6", "7"]);
    expect(run.ref5).toBe(5);
    expect(run.ref7).toBe(7);
  });

  it("the first ref drives the map through exactly the walked cells", () => {
    const transcript: string[] = [];
    const x = makeX(transcript);
    expect(streamRef(x, 5)).toBe(5);
    expect(transcript).toEqual(["0", "1", "2", "3", "4", "5"]);
  });

  it("the second ref memoizes: only the two new cells announce", () => {
    const transcript: string[] = [];
    const x = makeX(transcript);
    streamRef(x, 5);
    const before = transcript.length;
    expect(streamRef(x, 7)).toBe(7);
    expect(transcript.slice(before)).toEqual(["6", "7"]);
  });

  it("without memo-proc the second ref re-announces the cells it re-walks", () => {
    const transcript: string[] = [];
    const x = makeUnmemoizedX(transcript);
    expect(transcript).toEqual(["0"]);
    streamRef(x, 5);
    expect(transcript).toEqual(["0", "1", "2", "3", "4", "5"]);
    streamRef(x, 7);
    expect(transcript).toEqual(["0", "1", "2", "3", "4", "5", "1", "2", "3", "4", "5", "6", "7"]);
  });
});
