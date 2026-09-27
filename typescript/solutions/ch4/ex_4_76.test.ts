// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { frameOf, readQuery } from "../../packages/ch4/src/04-logic.js";
import { joinFrames, mergeFrames } from "./ex_4_76.js";

describe("4.76", () =>
  it("merges compatible frames and rejects conflicts", () => {
    const x = readQuery("?x"),
      y = readQuery("?y"),
      one = readQuery("1"),
      two = readQuery("2");
    const a = frameOf([[x, one]]);
    expect(mergeFrames(a, frameOf([[y, two]]))?.bindings).toHaveLength(2);
    expect(mergeFrames(a, frameOf([[x, two]]))).toBeUndefined();
    expect(joinFrames([a], [frameOf([[y, two]])])).toHaveLength(1);
  }));
