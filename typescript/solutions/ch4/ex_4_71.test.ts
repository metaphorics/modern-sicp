// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { delayedAppend, delayedInterleave } from "./ex_4_71.js";

describe("4.71", () => {
  it("defers append tail", () => expect(delayedAppend()).toEqual([1, 2, 2, 2]));
  it("alternates recursive choices", () => expect(delayedInterleave()).toEqual([1, 2, 2, 2, 3]));
});
