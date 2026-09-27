// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { appended, interleaved } from "./ex_4_72.js";

describe("4.72", () => {
  it("append starves later stream", () => expect(appended()).toEqual([1, 2, 3, 4, 5, 6]));
  it("interleave reaches later stream", () =>
    expect(interleaved()).toEqual([1, 100, 2, 200, 3, 4]));
});
