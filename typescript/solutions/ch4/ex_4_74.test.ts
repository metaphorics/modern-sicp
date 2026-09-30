// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { flatmapAgreement } from "./ex_4_74.js";

describe("exercise 4.74: Alyssa's simple stream-flatmap", () => {
  it("agrees with the interleaving flatmap on singleton/empty output", () => {
    const [simple, interleaved] = flatmapAgreement();
    expect(simple).toStrictEqual(['["Hacker", "Alyssa", "P"]', '["Tweakit", "Lem", "E"]']);
    expect(interleaved).toStrictEqual(simple);
  });
});
