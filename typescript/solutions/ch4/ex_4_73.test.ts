// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { flattenPrefix } from "./ex_4_73.js";

describe("4.73", () =>
  it("takes a prefix from infinite nested streams", () =>
    expect(flattenPrefix()).toEqual([1, 1, 2, 1, 3, 2])));
